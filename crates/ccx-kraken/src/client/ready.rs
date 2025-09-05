use std::marker::PhantomData;

use bon::Builder;
use ccx_lib::RequestError;
use serde::Deserialize;

use crate::api::error::KrakenApiError;
use crate::error::KrakenResult;
use crate::proto::{Request, RequestReadyToSend, Response};

use super::KrakenClient;
use super::meta::{KrakenError, KrakenResponseMeta};

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct SignData {
    sign: String,
    api_key: String,
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
    async fn send(self, client: &KrakenClient) -> KrakenResult<T::Response> {
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
            // https://docs.kraken.com/api/docs/guides/spot-rest-auth
            request = request
                .header("API-Key", sign.api_key.as_str())
                .header("API-Sign", sign.sign);
        }

        if let Some(body) = self.body {
            tracing::trace!(body);
            request = request.body(body);
        }

        handle_response_with_meta(request.send().await?).await
    }
}

async fn handle_response<T>(resp: reqwest::Response) -> Result<T, KrakenError>
where
    T: Response,
{
    let status = resp.status();

    if resp.status().is_success() {
        let full = resp.bytes().await?;

        tracing::trace!("Response: {}", String::from_utf8_lossy(&full));

        let response: RawResponse<T> = serde_json::from_slice(&full)?;
        if let Some(data) = response.result {
            Ok(data)
        } else {
            let error = response.error;
            tracing::error!(?error);
            Err(KrakenError::Api(error))
        }
    } else {
        Err(RequestError::Custom(anyhow::Error::from_boxed(
            status.canonical_reason().unwrap_or(status.as_str()).into(),
        ))
        .into())
    }
}

async fn handle_response_with_meta<T>(resp: reqwest::Response) -> KrakenResult<T>
where
    T: Response,
{
    let meta = KrakenResponseMeta::from_response(&resp);

    tracing::debug!(?meta);

    match handle_response(resp).await {
        Ok(payload) => Ok(meta.response(payload)),
        Err(error) => Err(meta.error(error)),
    }
}

#[derive(Deserialize)]
struct RawResponse<T> {
    result: Option<T>,
    error: KrakenApiError,
}
