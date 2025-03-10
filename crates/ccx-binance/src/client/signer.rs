use std::future::Future;

use ccx_lib::SignError;

pub trait BinanceSigner: Send {
    fn api_key(&self) -> &str;

    fn sign_request(&self, query: &str) -> impl Future<Output = Result<String, SignError>> + Send;
}

impl<T> BinanceSigner for &T
where
    T: BinanceSigner + Sync,
{
    fn api_key(&self) -> &str {
        (*self).api_key()
    }

    async fn sign_request(&self, query: &str) -> Result<String, SignError> {
        (*self).sign_request(query).await
    }
}
