use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::DelAllRequest;
use crate::types::DelAllResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_DEL_ALL;

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn del_all(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DelAllRequest>,
        ) -> LibResult<DelAllResponse> {
            self.client
                .post(API_DEL_ALL)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
