#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::CommitIncomingSettlementTransactionRequest;
use crate::types::CommitIncomingSettlementTransactionResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_COMMIT_INCOMING_SETTLEMENT_TRANSACTION;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn commit_incoming_settlement_transaction(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<CommitIncomingSettlementTransactionRequest>,
        ) -> LibResult<CommitIncomingSettlementTransactionResponse> {
            self.client
                .post(API_COMMIT_INCOMING_SETTLEMENT_TRANSACTION)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
