#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_ADD_INCOMING_SETTLEMENT_REQUEST;
use crate::types::AddIncomingSettlementRequest;
use crate::types::AddIncomingSettlementResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn add_incoming_settlement_request(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<AddIncomingSettlementRequest>,
        ) -> LibResult<AddIncomingSettlementResponse> {
            self.client
                .post(API_ADD_INCOMING_SETTLEMENT_REQUEST)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
