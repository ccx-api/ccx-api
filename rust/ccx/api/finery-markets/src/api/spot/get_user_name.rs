use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::GetUserNameRequest;
use crate::types::GetUserNameResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_GET_USER_NAME;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn get_user_name(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<GetUserNameRequest>,
        ) -> LibResult<GetUserNameResponse> {
            self.client
                .post(API_GET_USER_NAME)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
