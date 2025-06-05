use serde::Serialize;

use crate::client::ready::ReadyRequest;
use crate::client::signer::BitGoSigner;
use crate::proto::{Request, SignedRequest};

use super::conversion::to_request_content;
use super::meta::BitGoError;
use super::ready::SignData;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct Stamped<T> {
    request: T,
}

impl<T> Stamped<T> {
    pub fn new(request: T) -> Self {
        Stamped { request }
    }
}

impl<T: SignedRequest> Request for Stamped<T> {
    type Response = T::Response;

    const HTTP_METHOD: http::Method = T::HTTP_METHOD;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        self.request.path()
    }
}

impl<T> Stamped<T>
where
    T: SignedRequest + Send,
{
    #[tracing::instrument(skip_all)]
    pub async fn sign(self, signer: impl BitGoSigner) -> Result<ReadyRequest<T>, BitGoError> {
        let path = self.request.path();
        let content = to_request_content(&self.request)?;

        let sign_data = SignData::builder()
            .bearer_token(signer.token().to_string())
            .build();

        Ok(ReadyRequest::builder()
            .path(path.into())
            .query(content.query)
            .body(content.body)
            .sign_data(sign_data)
            .build())
    }
}
