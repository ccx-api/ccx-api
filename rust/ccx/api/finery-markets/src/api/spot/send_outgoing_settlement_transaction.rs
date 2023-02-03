use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::SendOutgoingSettlementTransactionRequest;
use crate::types::SendOutgoingSettlementTransactionResponse;
use crate::types::Time;
use crate::types::API_SEND_OUTGOING_SETTLEMENT_TRANSACTION;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn send_outgoing_settlement_transaction(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SendOutgoingSettlementTransactionRequest>,
        ) -> LibResult<SendOutgoingSettlementTransactionResponse> {
            self.client
                .post(API_SEND_OUTGOING_SETTLEMENT_TRANSACTION)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
