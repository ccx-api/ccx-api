use std::future::Future;

use ccx_lib::SignError;

pub trait MexcSigner: Send {
    fn api_key(&self) -> &str;

    fn sign_request(&self, query: &str) -> impl Future<Output = Result<String, SignError>> + Send;
}

impl<T> MexcSigner for &T
where
    T: MexcSigner + Sync,
{
    fn api_key(&self) -> &str {
        (*self).api_key()
    }

    async fn sign_request(&self, query: &str) -> Result<String, SignError> {
        (*self).sign_request(query).await
    }
}
