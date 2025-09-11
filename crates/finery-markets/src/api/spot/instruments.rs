#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::API_INSTRUMENTS;
use crate::types::InstrumentsRequest;
use crate::types::InstrumentsResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn instruments(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<InstrumentsRequest>,
        ) -> LibResult<InstrumentsResponse> {
            self.client
                .post(API_INSTRUMENTS)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
