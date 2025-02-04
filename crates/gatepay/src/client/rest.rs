use std::borrow::Cow;
use std::sync::Arc;
use std::time::Instant;

use ccx_api_lib::make_client;
use ccx_api_lib::Client;
use ccx_api_lib::ClientRequest;
use ccx_api_lib::Method;
use ccx_api_lib::PayloadError;
use ccx_api_lib::SendRequestError;
use smart_string::DisplayExt;
use smart_string::SmartString;
use thiserror::Error;
use uuid::Uuid;

use crate::api::ApiMethod;
use crate::api::GatepayApiError;
use crate::api::GatepayResult;
use crate::api::Request;
use crate::client::config::GatepayApiConfig;
use crate::client::nonce::Nonce;
use crate::client::signer::GatepaySigner;
use crate::client::signer::SignError;
use crate::util::dt_gatepay::DtGatepay;

#[derive(Debug, Error)]
pub enum CallError {
    #[error("Send request error: {0}")]
    SendRequest(#[from] SendRequestError),
    #[error("Payload error: {0}")]
    Payload(#[from] PayloadError),
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Gatepay API error: {0}")]
    GatepayApi(#[from] GatepayApiError),
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Validate error: {0}")]
    Validate(Cow<'static, str>),
    #[error("Sign error: {0}")]
    Sign(#[from] SignError),
    #[error("Call error: {0}")]
    Call(#[from] CallError),
}

impl RequestError {
    #[inline]
    pub fn validate<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        Self::Validate(msg.into())
    }
}

/// API client.
pub struct GatepayRestClient<S>
where
    S: GatepaySigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for GatepayRestClient<S>
where
    S: GatepaySigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner<S>
where
    S: GatepaySigner,
{
    config: GatepayApiConfig<S>,
}

pub struct GatepayRequest<R, S>
where
    R: Request,
    S: GatepaySigner,
{
    api_client: GatepayRestClient<S>,
    request: ClientRequest,
    body: String,
    _phantom: std::marker::PhantomData<R>,
}

pub struct GatepayPreparedRequest<R, S>
where
    R: Request,
    S: GatepaySigner,
{
    api_client: GatepayRestClient<S>,
    request: ClientRequest,
    body: String,
    timestamp: DtGatepay,
    nonce: Nonce,
    _phantom: std::marker::PhantomData<R>,
}

pub struct GatepaySignedRequest<R>
where
    R: Request,
{
    request: ClientRequest,
    body: String,
    _phantom: std::marker::PhantomData<R>,
}

impl<S> GatepayRestClient<S>
where
    S: GatepaySigner,
{
    pub fn new(config: GatepayApiConfig<S>) -> Self {
        let inner = Arc::new(ClientInner { config });
        Self { inner }
    }

    pub(super) fn rest_client(&self) -> Client {
        make_client(false, self.inner.config.proxy.as_ref())
    }

    pub fn client_id(&self) -> &str {
        self.inner.config.signer.client_id()
    }

    pub fn rest<R: Request>(&self, request: &R) -> GatepayRequest<R, S> {
        let body = match R::METHOD {
            ApiMethod::Get => "".to_string(),
            ApiMethod::Post => serde_json::to_string(request).unwrap(),
        };
        let method = match R::METHOD {
            ApiMethod::Get => Method::GET,
            ApiMethod::Post => Method::POST,
        };
        let version = R::VERSION.as_str();
        let url_base = self.inner.config.api_base.as_str();
        let slash = if url_base.ends_with('/') { "" } else { "/" };
        let url: SmartString<254> = format_args!("{url_base}{slash}{version}/{}", R::PATH).to_fmt();

        let request = self
            .rest_client()
            .request(method, url.as_str())
            .append_header(("Content-Type", "application/json"));

        let request = request.append_header((
            "X-GatePay-Certificate-SN",
            self.inner.config.signer.api_key(),
        ));

        let api_client = self.clone();

        GatepayRequest {
            api_client,
            request,
            body,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, S> GatepayRequest<R, S>
where
    R: Request,
    S: GatepaySigner,
{
    pub fn now(self) -> GatepayPreparedRequest<R, S> {
        let Self {
            api_client,
            request,
            body,
            _phantom,
        } = self;

        let nonce = Nonce::random();
        let timestamp = DtGatepay::now();
        let request = request
            .append_header(("X-GatePay-Timestamp", timestamp.timestamp_ms()))
            .append_header(("X-GatePay-Nonce", nonce.to_string()));

        GatepayPreparedRequest {
            api_client,
            request,
            body,
            timestamp,
            nonce,
            _phantom,
        }
    }
}

impl<R, S> GatepayPreparedRequest<R, S>
where
    R: Request,
    S: GatepaySigner,
{
    pub async fn sign(self) -> Result<GatepaySignedRequest<R>, SignError> {
        let Self {
            api_client,
            request,
            body,
            timestamp,
            nonce,
            _phantom,
        } = self;

        let sign = api_client
            .inner
            .config
            .signer
            .sign_api(timestamp, &nonce, &body)
            .await?;

        let request = request.append_header(("X-GatePay-Signature", sign.to_string()));

        Ok(GatepaySignedRequest {
            request,
            body,
            _phantom,
        })
    }
}

impl<R> GatepaySignedRequest<R>
where
    R: Request,
{
    pub async fn call(self) -> Result<R::Response, CallError> {
        let Self {
            request,
            body,
            _phantom,
        } = self;

        let request_id = Uuid::new_v4();

        log::debug!("[{request_id}]  Request body: {:?}", body);

        let tm = Instant::now();
        let mut res = request.send_body(body).await?;
        let d1 = tm.elapsed();
        let body = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;

        log::debug!(
            "[{request_id}]  Time elapsed:  request: {:0.1}ms + body: {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );

        if cfg!(debug_assertions) {
            let body = String::from_utf8_lossy(&body);
            log::debug!("[{request_id}]  Response body: {:?}", body);
        }

        let resp: GatepayResult<R::Response> = serde_json::from_slice(&body)?;
        Ok(match resp {
            GatepayResult::Success(resp) => resp.data,
            GatepayResult::Fail(err) => Err(err)?,
        })
    }
}
