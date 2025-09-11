use crate::api::spot::SpotApi;
use crate::error::ApiFineryError;
use crate::error::LibError;
use crate::error::LibResult;
use crate::types::API_GET_COUNTERPARTY_INFO;
use crate::types::GetCounterpartyInfoRequest;
use crate::types::GetCounterpartyInfoResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn get_counterparty_info(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<GetCounterpartyInfoRequest>,
        ) -> LibResult<GetCounterpartyInfoResponse> {
            self.client
                .post(API_GET_COUNTERPARTY_INFO)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
