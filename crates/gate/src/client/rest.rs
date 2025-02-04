use std::sync::Arc;
use std::time::Instant;

use ccx_api_lib::make_client;
use ccx_api_lib::ClientRequest;
use ccx_api_lib::Method;
use ccx_api_lib::PayloadError;
use ccx_api_lib::SendRequestError;
use chrono::Utc;
use smart_string::DisplayExt;
use smart_string::SmartString;
use thiserror::Error;
use uuid::Uuid;

use super::websocket::WebsocketStream;
use crate::api::ApiMethod;
use crate::api::GateApiError;
use crate::api::PrivateRequest;
use crate::api::Request;
use crate::client::config::GateApiConfig;
use crate::client::signer::GateSigner;
use crate::client::signer::SignError;
use crate::error::GateResult;

#[derive(Debug, Error)]
pub enum CallError {
    #[error("Send request error: {0}")]
    SendRequest(#[from] SendRequestError),
    #[error("Payload error: {0}")]
    Payload(#[from] PayloadError),
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Gate.io API error: {0}")]
    GateApi(#[from] GateApiError),
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Sign error: {0}")]
    Sign(#[from] SignError),
    #[error("Call error: {0}")]
    Call(#[from] CallError),
}

/// API client.
pub struct RestClient<S> {
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestClient<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner<S> {
    config: GateApiConfig<S>,
}

pub struct GateRequest<R, S> {
    api_client: RestClient<S>,
    request: ClientRequest,
    body: String,
    _phantom: std::marker::PhantomData<R>,
}

pub struct GatePreparedRequest<R, S> {
    api_client: RestClient<S>,
    request: ClientRequest,
    body: String,
    timestamp: i64,
    // query: SmartString<254>,
    _phantom: std::marker::PhantomData<R>,
}

pub struct GateSignedRequest<R> {
    request: ClientRequest,
    body: String,
    _phantom: std::marker::PhantomData<R>,
}

impl<S> RestClient<S> {
    pub fn new(config: GateApiConfig<S>) -> Self {
        let inner = Arc::new(ClientInner { config });
        Self { inner }
    }

    /// REST and Websocket client from `awc` crate
    pub(super) fn client(&self) -> awc::Client {
        make_client(false, self.inner.config.proxy.as_ref())
    }

    pub fn prepare_rest<R: Request>(&self, path: &str, request: &R) -> GateRequest<R, S> {
        let body = match R::METHOD {
            ApiMethod::Get | ApiMethod::Delete => "".to_string(),
            ApiMethod::Post | ApiMethod::Put => serde_json::to_string(request).unwrap(),
        };
        let method = match R::METHOD {
            ApiMethod::Get => Method::GET,
            ApiMethod::Post => Method::POST,
            ApiMethod::Put => Method::PUT,
            ApiMethod::Delete => Method::DELETE,
        };
        let version = R::VERSION.as_str();
        let url_base = self.inner.config.api_base.as_str();
        let slash = if url_base.ends_with('/') { "" } else { "/" };
        let url: SmartString<254> = format_args!("{url_base}{slash}{version}{path}").to_fmt();

        let mut req = self
            .client()
            .request(method, url.as_str())
            .append_header(("Accept", "application/json"))
            .append_header(("Content-Type", "application/json"));

        if let ApiMethod::Get = R::METHOD {
            req = req.query(request).unwrap();
        }

        let api_client = self.clone();

        GateRequest {
            api_client,
            request: req,
            body,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn websocket(&self) -> GateResult<WebsocketStream> {
        let url = self.inner.config.stream_base.clone();
        WebsocketStream::connect(self.clone(), url).await
    }
}

impl<R: Request, S> GateRequest<R, S> {
    pub fn with_current_timestamp(self) -> GatePreparedRequest<R, S> {
        let Self {
            api_client,
            request,
            body,
            _phantom,
        } = self;

        let timestamp = Utc::now().timestamp();
        let request = request.append_header(("Timestamp", timestamp));

        GatePreparedRequest {
            api_client,
            request,
            body,
            timestamp,
            _phantom,
        }
    }

    pub async fn call_unsigned(self) -> Result<R::Response, CallError> {
        let Self { request, body, .. } = self;

        let request_id = Uuid::new_v4();

        log::debug!("[{request_id}]  Request body: {:?}", body);

        let tm = Instant::now();
        let mut res = request.send_body(body).await?;
        let is_success = res.status().is_success();
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

        Ok(match is_success {
            true => serde_json::from_slice::<R::Response>(&body)?,
            false => Err(serde_json::from_slice::<GateApiError>(&body)?)?,
        })
    }
}

impl<R: Request + PrivateRequest, S: GateSigner> GatePreparedRequest<R, S> {
    pub async fn sign(self) -> Result<GateSignedRequest<R>, SignError> {
        let Self {
            api_client,
            request,
            body,
            timestamp,
            _phantom,
        } = self;

        let request_method = request.get_method().as_str();
        let (request_path, request_query) = request
            .get_uri()
            .path_and_query()
            .map_or(("", ""), |pq| (pq.path(), pq.query().unwrap_or("")));

        if cfg!(debug_assertions) {
            log::debug!(
                "request: {:?} «{}»",
                request.get_method().as_str(),
                request.get_uri(),
            );
            log::debug!("headers: {:?}", request.headers());
            log::debug!("path: {:?}", request_path);
            log::debug!("query: {:?}", request_query);
            log::debug!("timestamp: {:?}", timestamp);
            log::debug!("body: {:?}", body);
        }

        debug_assert!(
            request_path.starts_with("/api/v4/"),
            "Invalid request_path: {:?}",
            request.get_uri().path_and_query(),
        );

        let timestamp: SmartString = timestamp.to_fmt();
        log::debug!("timestamp: {:?}", timestamp.as_str());

        let signer = &api_client.inner.config.signer;

        let sign = signer
            .sign_api(
                request_method,
                request_path,
                request_query,
                &body,
                &timestamp,
            )
            .await?;

        let request = request
            .append_header(("KEY", signer.key()))
            .append_header(("SIGN", sign.as_str()));

        log::debug!("headers: {:?}", request.headers());

        Ok(GateSignedRequest {
            request,
            body,
            _phantom,
        })
    }
}

impl<R: Request> GateSignedRequest<R> {
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
        let is_success = res.status().is_success();
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

        Ok(match is_success {
            true => serde_json::from_slice::<R::Response>(&body)?,
            false => Err(serde_json::from_slice::<GateApiError>(&body)?)?,
        })
    }
}
