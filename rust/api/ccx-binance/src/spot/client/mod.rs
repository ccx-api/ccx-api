use std::sync::Arc;

use crate::config::ConnectionConfig;
use crate::spot::error::BinanceSpotError;
use crate::spot::error::BinanceSpotErrorResponse;
use crate::spot::proto::BinanceSpotPublic;
use crate::spot::proto::BinanceSpotReadyToSend;

mod credential;
mod recv_window;
mod signed;
mod signer;
mod stamped;
mod time_window;

pub use credential::BinanceSpotCredential;
pub use recv_window::RecvWindow;
pub use signed::SignedRequest;
pub use signer::BinanceSpotSigner;
pub use stamped::Stamped;
pub use time_window::TimeWindow;

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

    pub async fn send_public<T>(&self, request: T) -> Result<T::Response, BinanceSpotError>
    where
        T: BinanceSpotPublic,
    {
        let inner = &self.inner;
        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        let query = serde_urlencoded::to_string(&request)?;
        if !query.is_empty() {
            url.set_query(Some(&query));
        }

        let request = inner.client.request(T::HTTP_METHOD, url);
        let resp = request.send().await?;
        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            Err(resp.json::<BinanceSpotErrorResponse>().await?)?
        }
    }
}

async fn handle_response<T>(resp: reqwest::Response) -> Result<T, BinanceSpotError>
where
    T: serde::de::DeserializeOwned,
{
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(resp.json::<BinanceSpotErrorResponse>().await?)?
    }
}

impl<T> BinanceSpotReadyToSend<T> for T
where
    T: BinanceSpotPublic,
{
    async fn send(self, client: &BinanceSpotClient) -> Result<T::Response, BinanceSpotError> {
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
