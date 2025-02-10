use std::sync::Arc;

use crate::config::ConnectionConfig;
use crate::spot::error::BinanceSpotErrorResponse;
use crate::spot::error::BinanceSpotSendError;
use crate::spot::proto::BinanceSpotPublic;
use crate::spot::proto::BinanceSpotReadyToSend;

mod credential;
mod recv_window;
mod signed;
mod signer;
mod stamped;
mod time_window;
mod websocket;

pub use credential::BinanceSpotCredential;
pub use recv_window::RecvWindow;
pub use signed::SignedRequest;
pub use signer::BinanceSpotSigner;
pub use stamped::Stamped;
pub use time_window::TimeWindow;

pub use self::websocket::WebSocketClient;
pub use self::websocket::WebSocketConnectError;
use crate::spot::api::websocket::WebSocketBuilder;
use crate::spot::meta::BinanceSpotMeta;
use crate::spot::meta::BinanceSpotResponseMeta;

#[derive(Clone)]
pub struct BinanceSpotClient {
    inner: Arc<ClientInner>,
}

pub struct ClientInner {
    client: reqwest::Client,
    config: ConnectionConfig,
}

impl BinanceSpotClient {
    pub fn new(client: reqwest::Client, config: ConnectionConfig) -> Self {
        let inner = ClientInner { client, config };
        let inner = Arc::new(inner);
        BinanceSpotClient { inner }
    }

    pub fn config(&self) -> &ConnectionConfig {
        &self.inner.config
    }

    pub async fn send_public<T>(
        &self,
        request: T,
    ) -> Result<BinanceSpotResponseMeta<T::Response>, BinanceSpotSendError>
    where
        T: BinanceSpotPublic,
    {
        let inner = &self.inner;
        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        let query = serde_urlencoded::to_string(&request)?;
        if !query.is_empty() {
            url.set_query(Some(&query));
        }

        if cfg!(feature = "debug_communication") {
            println!("Request: {}", url);
        }

        let request = inner.client.request(T::HTTP_METHOD, url);
        handle_response(request.send().await?).await
    }

    pub fn websocket(&self) -> WebSocketBuilder {
        WebSocketBuilder::new(self.clone())
    }
}

async fn handle_response<T>(
    resp: reqwest::Response,
) -> Result<BinanceSpotResponseMeta<T>, BinanceSpotSendError>
where
    T: serde::de::DeserializeOwned,
{
    let meta = BinanceSpotMeta::from_response(&resp);
    if resp.status().is_success() {
        let full = resp.bytes().await?;
        if cfg!(feature = "debug_communication") {
            let string = String::from_utf8_lossy(&full);
            println!("Response: {}", string);
        };
        let payload = serde_json::from_slice(&full)?;
        Ok(BinanceSpotResponseMeta::new(meta, payload))
    } else {
        let body = resp.json::<BinanceSpotErrorResponse>().await;
        let error = async { Err::<(), _>(body?.into()) }.await.unwrap_err();
        Err(BinanceSpotSendError::new(error, Some(meta)))?
    }
}

impl<T> BinanceSpotReadyToSend<T> for T
where
    T: BinanceSpotPublic,
{
    async fn send(
        self,
        client: &BinanceSpotClient,
    ) -> Result<BinanceSpotResponseMeta<T::Response>, BinanceSpotSendError> {
        let inner = &client.inner;
        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        let query = serde_urlencoded::to_string(&self)?;
        if !query.is_empty() {
            url.set_query(Some(&query));
        }

        let request = inner.client.request(T::HTTP_METHOD, url);

        handle_response(request.send().await?).await
    }
}

// impl<T> BinanceSpotReadyToSend<T> for PrivateRequest<T>
// where
//     T: BinanceSpotPrivate,
// {
//     async fn send(self, client: &BinanceSpotClient) -> Result<T::Response, BinanceSpotError>
//     where
//         T: BinanceSpotPrivate,
//     {
//         let inner = &client.inner;
//         let mut url = inner.config.api_base.join(T::ENDPOINT)?;
//         let query = serde_urlencoded::to_string(&self)?;
//         if !query.is_empty() {
//             url.set_query(Some(&query));
//         }
//
//         let request = inner
//             .client
//             .request(T::HTTP_METHOD, url)
//             .header("X-MBX-APIKEY", self.api_key);
//
//         handle_response(request.send().await?).await
//     }
// }
