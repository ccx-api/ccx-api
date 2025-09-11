#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_SET_CLIMIT;
use crate::types::Nonce;
use crate::types::SetCLimitRequest;
use crate::types::SetCLimitResponse;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn set_climit(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SetCLimitRequest>,
        ) -> LibResult<SetCLimitResponse> {
            self.client
                .post(API_SET_CLIMIT)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
