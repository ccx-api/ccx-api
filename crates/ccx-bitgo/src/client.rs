use std::sync::Arc;

use ccx_lib::websocket::WebSocketConnectError;
use reqwest::{IntoUrl, RequestBuilder};
use signer::BitGoSigner;
use soketto::handshake::client::Header;
use websocket::WebSocketClient;

use crate::config::ConnectionConfig;

mod conversion;
pub mod credential;
pub mod meta;
pub mod public;
pub mod ready;
pub mod signer;
pub mod stamped;
pub mod websocket;

#[derive(Clone)]
pub struct BitGoClient {
    inner: Arc<ClientInner>,
}

pub(crate) struct ClientInner {
    client: reqwest::Client,
    config: ConnectionConfig,
}

impl BitGoClient {
    pub fn new(client: reqwest::Client, config: ConnectionConfig) -> Self {
        let inner = ClientInner { client, config };
        let inner = Arc::new(inner);
        BitGoClient { inner }
    }

    pub fn config(&self) -> &ConnectionConfig {
        &self.inner.config
    }

    #[tracing::instrument(skip_all, fields(method = %method))]
    pub(crate) fn request(&self, method: http::Method, url: impl IntoUrl) -> RequestBuilder {
        self.inner.client.request(method, url)
    }

    pub async fn websocket(
        &self,
        signer: impl BitGoSigner,
    ) -> Result<WebSocketClient, WebSocketConnectError> {
        let mut websocket_url = self.config().api_base.join("api/prime/trading/v1/ws")?;

        // change scheme to websocket
        websocket_url.set_scheme("wss").unwrap();

        let auth = format!("Bearer {}", signer.token());
        let auth_header = Header {
            name: "Authorization",
            value: auth.as_bytes(),
        };

        WebSocketClient::connect(&websocket_url, &[auth_header]).await
    }
}
