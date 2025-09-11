#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_LIMITS;
use crate::types::LimitsRequest;
use crate::types::LimitsResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn limits(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<LimitsRequest>,
        ) -> LibResult<LimitsResponse> {
            self.client
                .post(API_LIMITS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
