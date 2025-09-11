#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_ADD_OUTGOING_SETTLEMENT_TRANSACTION;
use crate::types::AddOutgoingSettlementTransactionRequest;
use crate::types::AddOutgoingSettlementTransactionResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn add_outgoing_settlement_transaction(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<AddOutgoingSettlementTransactionRequest>,
        ) -> LibResult<AddOutgoingSettlementTransactionResponse> {
            self.client
                .post(API_ADD_OUTGOING_SETTLEMENT_TRANSACTION)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
