use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::DealHistoryRequest;
use crate::types::DealHistoryResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_DEAL_HISTORY;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn deal_history(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DealHistoryRequest>,
        ) -> LibResult<DealHistoryResponse> {
            self.client
                .post(API_DEAL_HISTORY)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
