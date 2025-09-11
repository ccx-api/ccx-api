#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_POSITIONS;
use crate::types::Nonce;
use crate::types::PositionsRequest;
use crate::types::PositionsResponse;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn positions(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<PositionsRequest>,
        ) -> LibResult<PositionsResponse> {
            self.client
                .post(API_POSITIONS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
