use std::marker::PhantomData;

use bon::Builder;
use ccx_lib::RequestError;
use ccx_lib::http::is_json_response;

use crate::api::error::BitGoApiError;
use crate::error::BitGoResult;
use crate::proto::{Request, RequestReadyToSend, Response};

use super::BitGoClient;
use super::meta::{BitGoError, BitGoResponseMeta};

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct SignData {
    bearer_token: String,
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
    async fn send(self, client: &BitGoClient) -> BitGoResult<T::Response> {
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
            request = request.bearer_auth(sign.bearer_token);
        }

        if let Some(body) = self.body {
            tracing::trace!(body);
            request = request.body(body);
        }

        handle_response_with_meta(request.send().await?).await
    }
}

async fn handle_response<T>(resp: reqwest::Response) -> Result<T, BitGoError>
where
    T: Response,
{
    let status = resp.status();

    let is_json = is_json_response(&resp);

    if resp.status().is_success() {
        let full = resp.bytes().await?;

        tracing::trace!("Response: {}", String::from_utf8_lossy(&full));

        let response = serde_json::from_slice(&full)?;

        Ok(response)
    } else {
        if is_json {
            let full = resp.bytes().await?;

            tracing::error!("Error response: {}", String::from_utf8_lossy(&full));

            let error: BitGoApiError = serde_json::from_slice(&full)?;

            Err(error.into())
        } else {
            Err(RequestError::Custom(anyhow::Error::from_boxed(
                status.canonical_reason().unwrap_or(status.as_str()).into(),
            ))
            .into())
        }
    }
}

async fn handle_response_with_meta<T>(resp: reqwest::Response) -> BitGoResult<T>
where
    T: Response,
{
    let meta = BitGoResponseMeta::from_response(&resp);

    tracing::debug!(?meta);

    match handle_response(resp).await {
        Ok(payload) => Ok(meta.response(payload)),
        Err(error) => Err(meta.error(error)),
    }
}
