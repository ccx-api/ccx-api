use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use actix_http::encoding::Decoder;
use actix_http::{Payload, PayloadStream};
use awc::http::{HeaderValue, Method};
use awc::Connector;
use awc::{Client, ClientRequest, ClientResponse};
use serde::Serialize;

use super::config::*;
use crate::client::{WebsocketClient, WebsocketStream};
use crate::error::*;

/// API client.
#[derive(Clone, Default)]
pub struct RestClient {
    inner: Rc<ClientInner>,
}

#[derive(Default)]
struct ClientInner {
    config: Config,
}

pub struct RequestBuilder {
    api_client: RestClient,
    request: ClientRequest,
}

impl RestClient {
    pub fn new() -> Self {
        RestClient::default()
    }

    pub fn with_config(config: Config) -> Self {
        let inner = Rc::new(ClientInner { config });
        RestClient { inner }
    }

    pub(super) fn client(&self) -> Client {
        let timeout = Duration::from_secs(5);
        let connector = Connector::new().timeout(timeout).finish();
        Client::builder()
            .connector(connector)
            .timeout(timeout)
            .finish()
    }

    pub(super) fn client_h1(&self) -> Client {
        let mut cfg = rustls::ClientConfig::new();
        cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
        cfg.root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        let timeout = Duration::from_secs(5);
        let connector = Connector::new()
            .rustls(Arc::new(cfg))
            .timeout(timeout)
            .finish();

        Client::builder()
            .connector(connector)
            .timeout(timeout)
            .finish()
    }

    pub fn request(&self, method: Method, endpoint: &str) -> LibResult<RequestBuilder> {
        let url = self.inner.config.api_base.join(endpoint)?;
        println!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        Ok(RequestBuilder {
            api_client,
            request,
        })
    }

    pub fn get(&self, endpoint: &str) -> LibResult<RequestBuilder> {
        self.request(Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: &str) -> LibResult<RequestBuilder> {
        self.request(Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: &str) -> LibResult<RequestBuilder> {
        self.request(Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: &str) -> LibResult<RequestBuilder> {
        self.request(Method::DELETE, endpoint)
    }

    pub async fn web_socket(&self) -> LibResult<WebsocketClient> {
        let url = self.inner.config.stream_base.clone();
        Ok(WebsocketClient::connect(self.clone(), url).await?)
    }

    pub async fn web_socket2(&self) -> LibResult<WebsocketStream> {
        let url = self.inner.config.stream_base.clone();
        Ok(WebsocketStream::connect(self.clone(), url).await?)
    }
}

impl RequestBuilder {
    pub fn query_args<T: Serialize>(mut self, query: &T) -> LibResult<Self> {
        self.request = self.request.query(query)?;
        Ok(self)
    }

    pub fn query_arg<S: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: S,
        query: &T,
    ) -> LibResult<Self> {
        self.request = self.request.query(&[(name.as_ref(), query)])?;
        Ok(self)
    }

    pub fn try_query_arg<S: AsRef<str>, T: Serialize>(
        mut self,
        name: S,
        query: &Option<T>,
    ) -> LibResult<Self> {
        if let Some(val) = query {
            self.request = self.request.query(&[(name.as_ref(), val)])?;
        }
        Ok(self)
    }

    pub fn auth_header(mut self) -> LibResult<Self> {
        self.request = self.request.header(
            "X-MBX-APIKEY",
            HeaderValue::from_str(self.api_client.inner.config.cred.key.as_str())?,
        );
        Ok(self)
    }

    pub async fn send<V>(self) -> LibResult<V>
    where
        V: serde::de::DeserializeOwned,
    {
        let res = self.request.send().await?;
        let mut res = check_response(res)?;
        Ok(res.json().limit(16 * 1024 * 1024).await?)
    }
}

type AwcClientResponse = ClientResponse<Decoder<Payload<PayloadStream>>>;

fn check_response(res: AwcClientResponse) -> LibResult<AwcClientResponse> {
    use awc::http::StatusCode;

    match res.status() {
        StatusCode::OK => Ok(res),
        StatusCode::INTERNAL_SERVER_ERROR => Err(ServiceError::ServerError)?,
        StatusCode::SERVICE_UNAVAILABLE => Err(ServiceError::ServiceUnavailable)?,
        StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized)?,
        // StatusCode::BAD_REQUEST => {
        //     let error_json: BinanceContentError = response.json()?;
        //
        //     Err(ErrorKind::BinanceError(error_json.code, error_json.msg, response).into())
        // }
        s => Err(LibError::UnknownStatus(s))?,
    }
}
