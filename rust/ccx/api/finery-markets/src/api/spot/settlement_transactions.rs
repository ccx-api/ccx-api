use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::SettlementTransactionsRequest;
use crate::types::SettlementTransactionsResponse;
use crate::types::Time;
use crate::types::API_SETTLEMENT_TRANSACTIONS;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn settlement_transactions(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SettlementTransactionsRequest>,
        ) -> LibResult<SettlementTransactionsResponse> {
            self.client
                .post(API_SETTLEMENT_TRANSACTIONS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
