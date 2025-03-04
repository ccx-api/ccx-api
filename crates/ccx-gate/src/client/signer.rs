use bon::Builder;
use ccx_lib::SignError;

use crate::types::timestamp::Timestamp;

#[derive(Builder)]
#[non_exhaustive]
pub struct GateSignerPayload<'a> {
    pub method: http::Method,
    pub path: &'a str,
    #[builder(required)]
    pub query: Option<&'a str>,
    #[builder(required)]
    pub body: Option<&'a str>,
    pub timestamp: Timestamp,
}

pub trait GateSigner: Send {
    fn api_key(&self) -> &str;

    fn sign_request(
        &self,
        payload: GateSignerPayload,
    ) -> impl Future<Output = Result<String, SignError>> + Send;
}

impl<T> GateSigner for &T
where
    T: GateSigner + Sync,
{
    fn api_key(&self) -> &str {
        (*self).api_key()
    }

    async fn sign_request(&self, payload: GateSignerPayload<'_>) -> Result<String, SignError> {
        (*self).sign_request(payload).await
    }
}
