#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_DEL;
use crate::types::DelRequest;
use crate::types::DelResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn del(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DelRequest>,
        ) -> LibResult<DelResponse> {
            self.client
                .post(API_DEL)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
