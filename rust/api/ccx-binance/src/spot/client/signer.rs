use std::future::Future;

use crate::spot::error::BinanceSpotError;

pub trait BinanceSpotSigner: Send {
    fn api_key(&self) -> &str;

    fn sign_request(
        &self,
        query: &str,
    ) -> impl Future<Output = Result<String, BinanceSpotError>> + Send;
}

impl<T> BinanceSpotSigner for &T
where
    T: BinanceSpotSigner + Sync,
{
    fn api_key(&self) -> &str {
        (*self).api_key()
    }

    async fn sign_request(&self, query: &str) -> Result<String, BinanceSpotError> {
        (*self).sign_request(query).await
    }
}
