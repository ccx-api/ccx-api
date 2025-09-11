#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_DEL_OUTGOING_SETTLEMENT_TRANSACTION;
use crate::types::DelOutgoingSettlementTransactionRequest;
use crate::types::DelOutgoingSettlementTransactionResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn del_outgoing_settlement_transaction(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DelOutgoingSettlementTransactionRequest>,
        ) -> LibResult<DelOutgoingSettlementTransactionResponse> {
            self.client
                .post(API_DEL_OUTGOING_SETTLEMENT_TRANSACTION)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
