#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::BookRequest;
use crate::types::BookResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_BOOK;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn book(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<BookRequest>,
        ) -> LibResult<BookResponse> {
            let resp = self
                .client
                .post(API_BOOK)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await;
            log::debug!("XXX ccx_finery_markets book :: {:?}", resp);
            resp
        }
    }
}
