#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::SetLimitRequest;
use crate::types::SetLimitResponse;
use crate::types::Time;
use crate::types::API_SET_LIMIT;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn set_limit(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SetLimitRequest>,
        ) -> LibResult<SetLimitResponse> {
            self.client
                .post(API_SET_LIMIT)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
