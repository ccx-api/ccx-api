use serde::Serialize;

use crate::client::ready::ReadyRequest;
use crate::client::signer::KrakenSigner;
use crate::proto::{Request, SignedRequest};

use super::conversion::to_request_content;
use super::meta::KrakenError;
use super::ready::SignData;
use super::signer::{KrakenSignerPayload, Nonce};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct Stamped<T> {
    nonce: Nonce,
    #[serde(flatten)]
    request: T,
}

impl<T> Stamped<T> {
    pub fn new(request: T, nonce: Nonce) -> Self {
        Stamped { request, nonce }
    }
}

impl<T: SignedRequest> Request for Stamped<T> {
    type Response = T::Response;

    const HTTP_METHOD: http::Method = T::HTTP_METHOD;

    const ENDPOINT: &'static str = T::ENDPOINT;
}

impl<T> Stamped<T>
where
    T: SignedRequest + Send,
{
    #[tracing::instrument(skip_all)]
    pub async fn sign(self, signer: impl KrakenSigner) -> Result<ReadyRequest<T>, KrakenError> {
        let path = self.request.path();
        // nonce should be passed as the part of the body, so we use self here instead
        // of the underlying `request`
        let content = to_request_content(&self)?;

        let signer_payload = KrakenSignerPayload::builder()
            .path(&path)
            .body(content.body.as_deref().unwrap_or_default())
            .nonce(self.nonce)
            .build();

        let sign = signer.sign_request(signer_payload).await?;

        let sign_data = SignData::builder()
            .sign(sign)
            .api_key(signer.api_key().to_string())
            .build();

        Ok(ReadyRequest::builder()
            .path(path.into())
            .query(content.query)
            .body(content.body)
            .sign_data(sign_data)
            .build())
    }
}
