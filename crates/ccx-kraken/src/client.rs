use std::sync::Arc;

use ccx_lib::websocket::WebSocketConnectError;
use reqwest::{IntoUrl, RequestBuilder};
// use websocket::WebSocketClient;

use crate::config::ConnectionConfig;

mod conversion;
pub mod credential;
pub mod meta;
pub mod public;
pub mod ready;
pub mod signer;
pub mod stamped;
// pub mod websocket;

#[derive(Clone)]
pub struct KrakenClient {
    inner: Arc<ClientInner>,
}

pub(crate) struct ClientInner {
    client: reqwest::Client,
    config: ConnectionConfig,
}

impl KrakenClient {
    pub fn new(client: reqwest::Client, config: ConnectionConfig) -> Self {
        let inner = ClientInner { client, config };
        let inner = Arc::new(inner);
        KrakenClient { inner }
    }

    pub fn config(&self) -> &ConnectionConfig {
        &self.inner.config
    }

    #[tracing::instrument(skip_all, fields(method = %method))]
    pub(crate) fn request(&self, method: http::Method, url: impl IntoUrl) -> RequestBuilder {
        self.inner.client.request(method, url)
    }

    // pub async fn websocket(&self) -> Result<WebSocketClient, WebSocketConnectError> {
    //     WebSocketClient::connect(&self.config().websocket_base).await
    // }
}
