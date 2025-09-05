use std::marker::PhantomData;

use bon::Builder;

use crate::api::error::GateApiError;
use crate::error::GateResult;
use crate::proto::{Request, RequestReadyToSend, Response};
use crate::types::timestamp::Timestamp;

use super::GateClient;
use super::meta::{GateError, GateResponseMeta};

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct SignData {
    sign: String,
    api_key: String,
    timestamp: Timestamp,
}

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct ReadyRequest<T> {
    path: String,
    #[builder(required)]
    query: Option<String>,
    #[builder(required)]
    body: Option<String>,
    sign_data: Option<SignData>,
    #[builder(skip)]
    request_spec: PhantomData<T>,
}

impl<T> RequestReadyToSend<T> for ReadyRequest<T>
where
    T: Request,
{
    #[tracing::instrument(skip_all, fields(http_method = %T::HTTP_METHOD, endpoint = %self.path), err)]
    async fn send(self, client: &GateClient) -> GateResult<T::Response> {
        let mut url = client.config().api_base.join(&self.path)?;

        url.set_query(self.query.as_deref());

        tracing::debug!(
            method = %T::HTTP_METHOD,
            %url,
            with_body = self.body.is_some(),
            with_sign_data = self.sign_data.is_some(),
            "Making request"
        );

        let mut request = client.request(T::HTTP_METHOD, url);

        request = request
            .header(http::header::ACCEPT, "application/json")
            .header(http::header::CONTENT_TYPE, "application/json");

        if let Some(sign) = self.sign_data {
            // Docs for authorized request:
            // https://www.gate.io/docs/developers/apiv4/#apiv4-signed-request-requirements
            request = request
                .header("KEY", sign.api_key.as_str())
                .header("TIMESTAMP", sign.timestamp.to_string())
                .header("SIGN", sign.sign);
        }

        if let Some(body) = self.body {
            tracing::trace!(body);
            request = request.body(body);
        }

        handle_response_with_meta(request.send().await?).await
    }
}

async fn handle_response<T>(resp: reqwest::Response) -> Result<T, GateError>
where
    T: Response,
{
    if resp.status().is_success() {
        let full = resp.bytes().await?;

        tracing::trace!("Response: {}", String::from_utf8_lossy(&full));

        let payload = serde_json::from_slice(&full)?;

        Ok(payload)
    } else {
        let err = resp.json::<GateApiError>().await?;

        tracing::error!(?err);

        Err(GateError::Api(err))
    }
}

async fn handle_response_with_meta<T>(resp: reqwest::Response) -> GateResult<T>
where
    T: Response,
{
    let meta = GateResponseMeta::from_response(&resp);

    tracing::debug!(?meta);

    match handle_response(resp).await {
        Ok(payload) => Ok(meta.response(payload)),
        Err(error) => Err(meta.error(error)),
    }
}
