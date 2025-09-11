#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_ADD;
use crate::types::AddRequest;
use crate::types::AddResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn add(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<AddRequest>,
        ) -> LibResult<AddResponse> {
            self.client
                .post(API_ADD)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
