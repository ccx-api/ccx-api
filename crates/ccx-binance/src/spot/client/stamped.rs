use std::future::Future;

use serde::Serialize;

use crate::spot::client::signed::SignedRequest;
use crate::spot::client::BinanceSpotSigner;
use crate::spot::client::TimeWindow;
use crate::spot::error::BinanceSpotError;
use crate::spot::proto::BinanceSpotSigned;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct Stamped<T> {
    #[serde(flatten)]
    request: T,
    #[serde(flatten)]
    time_window: TimeWindow,
}

impl<T> Stamped<T> {
    pub(in crate::spot) fn new(request: T, time_window: TimeWindow) -> Self {
        Stamped {
            request,
            time_window,
        }
    }
}

impl<T> Stamped<T>
where
    T: BinanceSpotSigned + Send,
{
    pub fn sign(
        self,
        signer: impl BinanceSpotSigner,
    ) -> impl Future<Output = Result<SignedRequest<T>, BinanceSpotError>> + Send {
        use std::fmt::Write;

        async move {
            let mut query = serde_urlencoded::to_string(&self)?;

            let signature = signer.sign_request(&query).await?;
            let div = if query.is_empty() { "" } else { "&" };
            write!(query, "{div}signature={signature}")?;

            Ok(SignedRequest::new(query, signer.api_key()))
        }
    }
}
