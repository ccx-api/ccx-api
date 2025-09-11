#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_DEL_CLIMIT;
use crate::types::DelCLimitRequest;
use crate::types::DelCLimitResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn del_climit(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DelCLimitRequest>,
        ) -> LibResult<DelCLimitResponse> {
            self.client
                .post(API_DEL_CLIMIT)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
