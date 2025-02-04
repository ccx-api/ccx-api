#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::SettlementRequest;
use crate::types::SettlementResponse;
use crate::types::Time;
use crate::types::API_SETTLEMENT_REQUESTS;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn settlement_requests(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SettlementRequest>,
        ) -> LibResult<SettlementResponse> {
            self.client
                .post(API_SETTLEMENT_REQUESTS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
