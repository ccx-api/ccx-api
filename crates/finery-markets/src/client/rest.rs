use std::borrow::Cow;
use std::sync::Arc;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::BoxedPayloadStream;
use awc::http::Method;
use awc::http::StatusCode;
use awc::Client;
use awc::ClientRequest;
use awc::ClientResponse;
use ccx_api_lib::make_client;
use serde::Serialize;
use url::Url;

use super::WebSocket;
use crate::client::Config;
use crate::client::FinerySigner;
use crate::client::WsSender;
use crate::error::LibError;
use crate::error::LibResult;
use crate::error::ServiceError;
use crate::types::ApiError;
use crate::types::Nonce;
use crate::types::Payload;
use crate::types::Time;
use crate::Proxy;

pub struct RestClient<S>
where
    S: FinerySigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestClient<S>
where
    S: FinerySigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub(crate) struct ClientInner<S>
where
    S: FinerySigner,
{
    config: Config<S>,
}

pub struct RequestBuilder<S, J>
where
    S: FinerySigner,
    J: Serialize,
{
    api_client: RestClient<S>,
    request: ClientRequest,
    nonce: Option<Nonce>,
    time: Option<Time>,
    content: Option<J>,
}

impl<S> RestClient<S>
where
    S: FinerySigner,
{
    pub fn new(config: Config<S>) -> Self {
        let inner = Arc::new(ClientInner { config });
        RestClient { inner }
    }

    fn client_(&self, h1_only: bool) -> Client {
        make_client(h1_only, self.inner.config.proxy.as_ref())
    }

    pub(crate) fn stream_url(&self) -> Url {
        self.inner.config.stream_base.clone()
    }

    pub(crate) fn proxy(&self) -> Option<Proxy> {
        self.inner.config.proxy.clone()
    }

    pub(super) fn signer(&self) -> &S {
        self.inner.config.signer()
    }

    pub(super) fn key(&self) -> &str {
        self.inner.config.api_key()
    }

    pub async fn web_socket(&self, tx: WsSender, nonce: Nonce, time: Time) -> LibResult<WebSocket> {
        WebSocket::connect(tx, self.clone(), nonce, time).await
    }

    pub(super) fn client(&self) -> Client {
        self.client_(false)
    }

    #[allow(dead_code)]
    pub(super) fn client_h1(&self) -> Client {
        self.client_(true)
    }

    pub fn request<T: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> LibResult<RequestBuilder<S, T>> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        Ok(RequestBuilder {
            api_client,
            request,
            nonce: None,
            time: None,
            content: None,
        })
    }

    pub fn post<T: Serialize>(&self, endpoint: &str) -> LibResult<RequestBuilder<S, T>> {
        self.request(Method::POST, endpoint)
    }
}

impl<S, J> RequestBuilder<S, J>
where
    S: FinerySigner,
    J: Serialize,
    J: std::fmt::Debug,
    J: Clone,
{
    fn header_key(mut self) -> LibResult<Self> {
        self.request = self
            .request
            .append_header(("EFX-Key", self.api_client.inner.config.api_key()));
        Ok(self)
    }

    async fn header_sign(mut self) -> LibResult<Self> {
        let path = self.request.get_uri().path();
        let method = basename(path);
        let payload = self.payload()?;
        let payload = serde_json::to_string(&payload)?;

        log::debug!("header_sign method :: {}; payload :: {}", method, payload);
        let content = format!("{}{}", method, payload);

        let signature = self.api_client.signer().sign_data(&content).await?;

        log::debug!("header_sign signature :: {}", signature);

        self.request = self.request.append_header(("EFX-Sign", signature));
        Ok(self)
    }

    fn header_content_type(mut self) -> LibResult<Self> {
        self.request = self.request.append_header(("Content-Type", "text/html"));
        Ok(self)
    }

    pub fn nonce(mut self, nonce: Nonce) -> LibResult<Self> {
        self.nonce = Some(nonce);
        Ok(self)
    }

    pub fn time(mut self, time: Time) -> LibResult<Self> {
        self.time = Some(time);
        Ok(self)
    }

    pub fn content(mut self, request: J) -> LibResult<Self> {
        self.content = Some(request);
        Ok(self)
    }

    fn payload(&self) -> LibResult<Payload<J>> {
        Ok(Payload {
            content: self
                .content
                .clone()
                .ok_or_else(|| LibError::other("Unknown request content."))?,
            nonce: self
                .nonce
                .ok_or_else(|| LibError::other("Unknown request nonce."))?,
            time: self
                .time
                .ok_or_else(|| LibError::other("Unknown request timestamp."))?,
        })
    }

    pub async fn send<V>(mut self) -> LibResult<V>
    where
        V: serde::de::DeserializeOwned,
    {
        self = self.header_key()?;
        self = self.header_content_type()?;
        self = self.header_sign().await?;

        log::debug!(
            "send request :: {}  {}",
            self.request.get_method(),
            self.request.get_uri()
        );

        let tm = Instant::now();
        let mut res = {
            let payload = self.payload()?;
            log::debug!("send request :: {:?} :: {:?}", Time::now(), payload);
            self.request.send_json(&payload).await?
        };
        log::debug!("Time::now() :: {:?}", Time::now());
        let d1 = tm.elapsed();
        let resp = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;
        log::debug!(
            "Request time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );

        log::debug!("Response: {} «{:#?}»", res.status(), resp);
        check_response(res, &resp)?;
        match serde_json::from_slice(&resp) {
            Ok(json) => Ok(json),
            Err(err) => Err(LibError::Json(err)),
        }
    }
}

type AwcClientResponse = ClientResponse<Decoder<actix_http::Payload<BoxedPayloadStream>>>;

fn check_response(res: AwcClientResponse, resp: &[u8]) -> LibResult<AwcClientResponse> {
    match res.status() {
        StatusCode::OK => Ok(res),
        StatusCode::INTERNAL_SERVER_ERROR => Err(ServiceError::ServerError.into()),
        StatusCode::SERVICE_UNAVAILABLE => Err(ServiceError::ServiceUnavailable.into()),
        StatusCode::BAD_REQUEST => match serde_json::from_slice::<ApiError>(resp) {
            Ok(api_error) => Err(LibError::ApiError(api_error.error)),
            Err(err) => Err(LibError::Json(err)),
        },
        s => Err(LibError::UnknownStatus(s)),
    }
}

pub fn basename(path: &str) -> Cow<'_, str> {
    let mut pieces = path.rsplitn(2, |c| c == '/' || c == '\\');
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}
