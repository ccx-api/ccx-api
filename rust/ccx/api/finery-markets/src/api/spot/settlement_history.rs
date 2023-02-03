use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::SettlementHistoryRequest;
use crate::types::SettlementHistoryResponse;
use crate::types::Time;
use crate::types::API_SETTLEMENT_HISTORY;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn settlement_history(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<SettlementHistoryRequest>,
        ) -> LibResult<SettlementHistoryResponse> {
            self.client
                .post(API_SETTLEMENT_HISTORY)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
