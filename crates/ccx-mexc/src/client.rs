use std::sync::Arc;

use crate::config::ConnectionConfig;
use crate::error::MexcApiError;
use crate::error::MexcErrorWithMeta;

mod credential;
pub mod meta;
pub mod public;
mod recv_window;
mod signed_ready;
mod signer;
mod stamped;
mod time_window;
// mod websocket;

use ccx_lib::http::is_json_response;
pub use credential::MexcCredential;
pub use recv_window::RecvWindow;
use reqwest::{IntoUrl, RequestBuilder};
pub use signed_ready::SignedReadyRequest;
pub use signer::MexcSigner;
pub use stamped::Stamped;
pub use time_window::TimeWindow;

// pub use self::websocket::WebSocketClient;
// use crate::api::spot::websocket::WebSocketBuilder;
use crate::client::meta::MexcResponseMeta;
use crate::client::meta::MexcResponseWithMeta;

#[derive(Clone)]
pub struct MexcClient {
    inner: Arc<ClientInner>,
}

pub struct ClientInner {
    client: reqwest::Client,
    config: ConnectionConfig,
}

impl MexcClient {
    pub fn new(client: reqwest::Client, config: ConnectionConfig) -> Self {
        let inner = ClientInner { client, config };
        let inner = Arc::new(inner);
        MexcClient { inner }
    }

    pub fn config(&self) -> &ConnectionConfig {
        &self.inner.config
    }

    #[tracing::instrument(skip_all, fields(method = %method))]
    pub(crate) fn request(&self, method: http::Method, url: impl IntoUrl) -> RequestBuilder {
        self.inner.client.request(method, url)
    }

    // TODO: websocket API is not implemented yet
    // The implementation will require to integrate protobuf definitions
    // for the mexc binary messages
    // see [docs](https://mexcdevelop.github.io/apidocs/spot_v3_en/#websocket-market-streams)
    // pub fn websocket(&self) -> WebSocketBuilder {
    //     WebSocketBuilder::new(self.clone())
    // }
}

async fn handle_response<T>(
    resp: reqwest::Response,
) -> Result<MexcResponseWithMeta<T>, MexcErrorWithMeta>
where
    T: serde::de::DeserializeOwned,
{
    let meta = MexcResponseMeta::from_response(&resp);
    if resp.status().is_success() {
        let is_json = is_json_response(&resp);
        let full = resp.bytes().await?;

        tracing::trace!("Response: {}", String::from_utf8_lossy(&full));

        // some of the mexc methods return empty strings and not json content
        // try to parse as json if it's specified so
        let payload = if is_json {
            serde_json::from_slice(&full)?
        } else {
            serde_urlencoded::from_bytes(&full)?
        };

        Ok(MexcResponseWithMeta::new(meta, payload))
    } else {
        let body = resp.json::<MexcApiError>().await;
        let error = async { Err::<(), _>(body?.into()) }.await.unwrap_err();
        Err(MexcErrorWithMeta::new(error, Some(meta)))?
    }
}
