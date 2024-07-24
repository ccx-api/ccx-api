use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::GetSubaccountsRequest;
use crate::types::GetSubaccountsResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_GET_SUBACCOUNTS;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn get_subaccounts(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<GetSubaccountsRequest>,
        ) -> LibResult<GetSubaccountsResponse> {
            self.client
                .post(API_GET_SUBACCOUNTS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
