use bon::Builder;
use ccx_lib::SignError;

pub type Nonce = u64;

#[derive(Builder)]
#[non_exhaustive]
pub struct KrakenSignerPayload<'a> {
    pub path: &'a str,
    pub body: &'a str,
    pub nonce: Nonce,
}

pub trait KrakenSigner: Send {
    fn api_key(&self) -> &str;

    fn sign_request(
        &self,
        payload: KrakenSignerPayload,
    ) -> impl Future<Output = Result<String, SignError>> + Send;
}

impl<T> KrakenSigner for &T
where
    T: KrakenSigner + Sync,
{
    fn api_key(&self) -> &str {
        (*self).api_key()
    }

    async fn sign_request(&self, payload: KrakenSignerPayload<'_>) -> Result<String, SignError> {
        (*self).sign_request(payload).await
    }
}
