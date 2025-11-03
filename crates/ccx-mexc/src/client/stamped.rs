use std::future::Future;

use serde::Serialize;

use crate::client::MexcSigner;
use crate::client::TimeWindow;
use crate::client::signed_ready::SignedReadyRequest;
use crate::error::MexcError;
use crate::proto::SignedRequest;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct Stamped<T> {
    #[serde(flatten)]
    request: T,
    #[serde(flatten)]
    time_window: TimeWindow,
}

impl<T> Stamped<T> {
    pub fn new(request: T, time_window: TimeWindow) -> Self {
        Stamped {
            request,
            time_window,
        }
    }
}

impl<T> Stamped<T>
where
    T: SignedRequest + Send,
{
    pub fn sign(
        self,
        signer: impl MexcSigner,
    ) -> impl Future<Output = Result<SignedReadyRequest<T>, MexcError>> + Send {
        use std::fmt::Write;

        async move {
            let mut query = serde_html_form::to_string(&self)?;

            let signature = signer.sign_request(&query).await?;
            let div = if query.is_empty() { "" } else { "&" };
            write!(query, "{div}signature={signature}")?;

            Ok(SignedReadyRequest::new(query, signer.api_key()))
        }
    }
}
