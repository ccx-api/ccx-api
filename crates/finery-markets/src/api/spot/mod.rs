use std::time::Duration;

use actix::clock::sleep;
use ccx_api_lib::env_var_with_prefix;
use ccx_api_lib::ApiCred;
use ccx_api_lib::Proxy;
use futures::channel::mpsc;
use futures::future::select;
use futures::future::Either;
use futures::StreamExt;
use url::Url;

mod add;
mod add_incoming_settlement_request;
mod add_outgoing_settlement_transaction;
mod book;
mod climits;
mod commit_incoming_settlement_transaction;
mod deal_history;
mod del;
mod del_all;
mod del_climit;
mod del_incoming_settlement_cp_request;
mod del_incoming_settlement_request;
mod del_limit;
mod del_outgoing_settlement_transaction;
mod get_subaccounts;
mod get_counterparty_info;
mod instruments;
mod limits;
mod positions;
mod send_outgoing_settlement_transaction;
mod set_climit;
mod set_limit;
mod settlement_history;
mod settlement_requests;
mod settlement_transaction_history;
mod settlement_transactions;

use crate::client::Config;
use crate::client::FinerySigner;
use crate::client::RestClient;
use crate::client::WebSocket;
use crate::client::WsReceiver;
use crate::client::CCX_FINERY_API_PREFIX;
use crate::error::LibError;
use crate::error::LibResult;
use crate::types::ModRequest;
use crate::types::ModResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_MOD;

pub const API_BASE: &str = "https://trade.finerymarkets.com/api";
pub const STREAM_BASE: &str = "wss://trade.finerymarkets.com/ws";

pub const API_BASE_TESTNET: &str = "https://test.finerymarkets.com/api";
pub const STREAM_BASE_TESTNET: &str = "wss://test.finerymarkets.com/ws";

const CHANNEL_BUFFER_SIZE: usize = 1;

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(3);

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    #[derive(Clone)]
    pub struct SpotApi<S: FinerySigner> {
        pub client: RestClient<S>,
    }

    impl<S> SpotApi<S>
    where
        S: FinerySigner,
    {
        pub fn new(signer: S, testnet: bool, proxy: Option<Proxy>) -> Self {
            let (api_base, stream_base) = if testnet {
                (
                    Url::parse(API_BASE_TESTNET).unwrap(),
                    Url::parse(STREAM_BASE_TESTNET).unwrap(),
                )
            } else {
                (
                    Url::parse(API_BASE).unwrap(),
                    Url::parse(STREAM_BASE).unwrap(),
                )
            };
            SpotApi::with_config(Config::new(signer, api_base, stream_base, proxy))
        }

        pub fn from_env() -> SpotApi<ApiCred> {
            let testnet = Config::<S>::env_var("TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(CCX_FINERY_API_PREFIX);
            SpotApi::new(
                ApiCred::from_env_with_prefix(CCX_FINERY_API_PREFIX),
                testnet,
                proxy,
            )
        }

        pub fn from_env_with_prefix(prefix: &str) -> SpotApi<ApiCred> {
            let testnet = env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            SpotApi::new(ApiCred::from_env_with_prefix(prefix), testnet, proxy)
        }

        pub fn with_config(config: Config<S>) -> Self {
            let client = RestClient::new(config);
            SpotApi { client }
        }

        pub async fn call_mod(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<ModRequest>,
        ) -> LibResult<ModResponse> {
            self.client
                .post(API_MOD)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }

        pub async fn ws(&self, nonce: Nonce, time: Time) -> LibResult<(WebSocket, WsReceiver)> {
            let (tx, mut rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
            let ws = self.client.web_socket(tx, nonce, time).await?;
            let ws_handle = actix_rt::spawn(async move {
                let msg = rx.next().await;
                // log::debug!("msg :: {:?}", msg);
                // sleep(Duration::from_secs(5)).await;
                match msg {
                    Some(Ok(message)) => match message.connected()? {
                        true => Ok(rx),
                        false => Err(LibError::other("Finery WS not connected.")),
                    },
                    Some(Err(error)) => {
                        log::debug!("message error :: {:?}", error);
                        Err(error)
                    }
                    None => {
                        log::debug!("channel was closed");
                        Err(LibError::other("channel was closed."))
                    }
                }
            });
            let timeout_handle = actix_rt::spawn(async {
                sleep(CONNECTION_TIMEOUT).await;
            });
            match select(ws_handle, timeout_handle).await {
                Either::Left((rx_res, _)) => rx_res
                    .map_err(|e| {
                        LibError::other(format!("Failed awaiting connection status: {:?}", e))
                    })?
                    .map(|rx| (ws, rx)),
                Either::Right(_) => {
                    ws.close().await;
                    Err(LibError::other("Finery WS not connected."))
                }
            }
        }
    }
}
