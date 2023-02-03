use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::CLimitsRequest;
use crate::types::CLimitsResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_CLIMITS;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn climits(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<CLimitsRequest>,
        ) -> LibResult<CLimitsResponse> {
            self.client
                .post(API_CLIMITS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
