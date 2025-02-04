#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::SettlementTransactionHistoryRequest;
use crate::types::SettlementTransactionHistoryResponse;
use crate::types::Time;
use crate::types::API_SETTLEMENT_TRANSACTION_HISTORY;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn settlement_transaction_history(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SettlementTransactionHistoryRequest>,
        ) -> LibResult<SettlementTransactionHistoryResponse> {
            self.client
                .post(API_SETTLEMENT_TRANSACTION_HISTORY)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
