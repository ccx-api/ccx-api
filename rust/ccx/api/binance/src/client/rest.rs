use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::{Payload, PayloadStream};
use awc::http::{HeaderValue, Method, Uri};
use awc::Connector;
use awc::{Client, ClientRequest, ClientResponse};
use hmac::{Hmac, Mac, NewMac};
use serde::Serialize;
use sha2::Sha256;

use super::*;
use crate::client::limits::UsedRateLimits;
use crate::client::{WebsocketClient, WebsocketStream};
use crate::error::*;
use crate::proto::TimeWindow;

/// API client.
#[derive(Clone)]
pub struct RestClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    config: Config,
}

pub struct RequestBuilder {
    api_client: RestClient,
    request: ClientRequest,
    sign: Option<TimeWindow>,
    secret: Vec<u8>,
}

impl RestClient {
    pub fn new(config: Config) -> Self {
        let inner = Arc::new(ClientInner { config });
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
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        let secret = api_client.inner.config.cred.secret.as_bytes().to_vec();
        Ok(RequestBuilder {
            api_client,
            request,
            sign: None,
            secret,
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
    pub fn uri(&self) -> String {
        self.request.get_uri().to_string()
    }

    pub fn query_args<T: Serialize>(mut self, query: &T) -> LibResult<Self> {
        self.request = self.request.query(query)?;
        Ok(self)
    }

    pub fn query_arg<S: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: S,
        query: &T,
    ) -> LibResult<Self> {
        let mut parts = self.request.get_uri().clone().into_parts();

        if let Some(path_and_query) = parts.path_and_query {
            let mut buf = path_and_query.path().to_string();
            buf.push('?');
            match path_and_query.query().unwrap_or("") {
                "" => {}
                old_query => {
                    buf.push_str(old_query);
                    buf.push('&');
                }
            }
            buf.push_str(&serde_urlencoded::to_string(&[(name.as_ref(), query)])?);
            parts.path_and_query = buf.parse().ok();
            let uri = Uri::from_parts(parts).map_err(|e| LibError::other(format!("{:?}", e)))?;
            self.request = self.request.uri(uri);
        }

        Ok(self)
    }

    pub fn try_query_arg<S: AsRef<str>, T: Serialize>(
        self,
        name: S,
        query: &Option<T>,
    ) -> LibResult<Self> {
        match query {
            Some(val) => self.query_arg(name, val),
            None => Ok(self),
        }
    }

    pub fn auth_header(mut self) -> LibResult<Self> {
        self.request = self.request.header(
            "X-MBX-APIKEY",
            HeaderValue::from_str(self.api_client.inner.config.cred.key.as_str())?,
        );
        Ok(self)
    }

    pub fn signed(mut self, time_window: impl Into<TimeWindow>) -> LibResult<Self> {
        self.sign = Some(time_window.into());
        self.auth_header()
    }

    pub async fn send<V>(mut self) -> LibResult<V>
    where
        V: serde::de::DeserializeOwned,
    {
        self = if let Some(sign) = self.sign.clone() {
            self = self.query_arg("timestamp", &sign.timestamp())?;
            let recv_window = sign.recv_window();
            if !recv_window.is_default() {
                self = self.query_arg("recvWindow", &*recv_window)?;
            }
            self.sign()?
        } else {
            self
        };
        log::debug!(
            "{}  {}",
            self.request.get_method(),
            self.request.get_uri(),
        );
        let tm = Instant::now();
        let mut res = self.request.send().await?;
        let d1 = tm.elapsed();
        let resp = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;
        log::debug!(
            "Request time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );
        log::debug!("Response: {} «{}»", res.status(), String::from_utf8_lossy(&resp));
        if let Err(err) = check_response(res) {
            // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
            Err(err)?
        };
        match serde_json::from_slice(&resp) {
            Ok(json) => Ok(json),
            Err(err) => {
                // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
                Err(err)?
            }
        }
    }

    pub async fn send_no_responce(mut self) -> LibResult<()>
    {
        self = if let Some(sign) = self.sign.clone() {
            self = self.query_arg("timestamp", &sign.timestamp())?;
            let recv_window = sign.recv_window();
            if !recv_window.is_default() {
                self = self.query_arg("recvWindow", &*recv_window)?;
            }
            self.sign()?
        } else {
            self
        };
        log::debug!(
            "{}  {}",
            self.request.get_method(),
            self.request.get_uri(),
        );
        let tm = Instant::now();
        let mut res = self.request.send().await?;
        let d1 = tm.elapsed();
        let resp = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;
        log::debug!(
            "Request time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );
        log::debug!("Response: {} «{}»", res.status(), String::from_utf8_lossy(&resp));
        if let Err(err) = check_response(res) {
            // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
            Err(err)?
        };
        Ok(())
    }

    fn sign(self) -> LibResult<Self> {
        let query = self.request.get_uri().query().unwrap_or("");
        let signature = sign(query, &self.secret);
        self.query_arg("signature", &signature)
    }
}

fn sign(query: &str, secret: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_varkey(secret).expect("HMAC can take key of any size");
    mac.update(query.as_bytes());

    let res = mac.finalize().into_bytes();
    format!("{:x}", res)
}

type AwcClientResponse = ClientResponse<Decoder<Payload<PayloadStream>>>;

fn check_response(res: AwcClientResponse) -> LibResult<AwcClientResponse> {
    use awc::http::StatusCode;

    let used_rate_limits = UsedRateLimits::from_headers(res.headers());

    log::debug!("  used_rate_limits:  {:?}", used_rate_limits);

    match res.status() {
        StatusCode::OK => Ok(res),
        StatusCode::INTERNAL_SERVER_ERROR => Err(ServiceError::ServerError)?,
        StatusCode::SERVICE_UNAVAILABLE => Err(ServiceError::ServiceUnavailable)?,
        StatusCode::UNAUTHORIZED => Err(RequestError::Unauthorized)?,
        // StatusCode::BAD_REQUEST => {
        //     let error_json: BinanceContentError = response.json()?;
        //
        //     Err(ErrorKind::BinanceError(error_json.code, error_json.msg, response).into())
        // }
        s => Err(LibError::UnknownStatus(s))?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sign() {
        let query = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&\
                    recvWindow=5000&timestamp=1499827319559";
        let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let res = sign(query, key.as_bytes());
        assert_eq!(
            res,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        )
    }
}
