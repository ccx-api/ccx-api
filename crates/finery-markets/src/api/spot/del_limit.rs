#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::DelLimitRequest;
use crate::types::DelLimitResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_DEL_LIMIT;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn del_limit(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DelLimitRequest>,
        ) -> LibResult<DelLimitResponse> {
            self.client
                .post(API_DEL_LIMIT)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
