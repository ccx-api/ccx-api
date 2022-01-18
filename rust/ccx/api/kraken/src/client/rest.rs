use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::{Payload, PayloadStream};
use awc::http::{HeaderValue, Method, Uri};
use awc::Connector;
use awc::{Client, ClientRequest, ClientResponse};
use hmac::{Hmac, Mac, NewMac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};

use ccx_api_lib::SocksConnector;

use super::*;
// use crate::client::limits::UsedRateLimits;
// use crate::client::{WebsocketClient, WebsocketStream};
use crate::error::*;
// use crate::proto::TimeWindow;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

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
    sign: Option<(Nonce,)>,
    signer: Signer,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct KrakenApiAnswer<T> {
    result: Option<T>,
    error: Vec<String>,
}

impl<T> KrakenApiAnswer<T>
where
    T: Deserialize<'static>,
{
    fn into_api_result(self) -> KrakenApiResult<T> {
        let error = KrakenApiError(self.error.into_iter().map(ApiError::from_string).collect());
        match self.result {
            Some(result) => Ok((result, error)),
            None => Err(error)?,
        }
    }
}

impl RestClient {
    pub fn new(config: Config) -> Self {
        let inner = Arc::new(ClientInner { config });
        RestClient { inner }
    }

    fn client_(&self, h1_only: bool) -> Client {
        let cfg = Self::client_config(h1_only);
        match self.inner.config.proxy.as_ref() {
            Some(proxy) => self.client_with_proxy(cfg, proxy),
            None => self.client_without_proxy(cfg),
        }
    }

    pub(super) fn client(&self) -> Client {
        self.client_(false)
    }

    #[allow(dead_code)]
    pub(super) fn client_h1(&self) -> Client {
        self.client_(true)
    }

    fn client_config(h1_only: bool) -> Arc<rustls::ClientConfig> {
        let mut cfg = rustls::ClientConfig::new();
        if h1_only {
            cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
        }
        cfg.root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        Arc::new(cfg)
    }

    fn client_without_proxy(&self, cfg: Arc<rustls::ClientConfig>) -> Client {
        let connector = Connector::new()
            .rustls(cfg)
            .timeout(CONNECT_TIMEOUT)
            .finish();
        Client::builder()
            .connector(connector)
            .timeout(CLIENT_TIMEOUT)
            .finish()
    }

    fn client_with_proxy(&self, cfg: Arc<rustls::ClientConfig>, proxy: &Proxy) -> Client {
        let connector = Connector::new()
            .rustls(cfg)
            .connector(SocksConnector::new(proxy.addr()))
            .timeout(CONNECT_TIMEOUT)
            .finish();
        Client::builder()
            .connector(connector)
            .timeout(CLIENT_TIMEOUT)
            .finish()
    }

    pub fn request(&self, method: Method, endpoint: &str) -> KrakenResult<RequestBuilder> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        let signer = api_client.inner.config.signer().clone();
        Ok(RequestBuilder {
            api_client,
            request,
            sign: None,
            signer,
            body: String::new(),
        })
    }

    pub fn get(&self, endpoint: &str) -> KrakenResult<RequestBuilder> {
        self.request(Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: &str) -> KrakenResult<RequestBuilder> {
        self.request(Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: &str) -> KrakenResult<RequestBuilder> {
        self.request(Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: &str) -> KrakenResult<RequestBuilder> {
        self.request(Method::DELETE, endpoint)
    }

    // pub async fn web_socket(&self) -> KrakenResult<WebsocketClient> {
    //     let url = self.inner.config.stream_base.clone();
    //     Ok(WebsocketClient::connect(self.clone(), url).await?)
    // }
    //
    // pub async fn web_socket2(&self) -> KrakenResult<WebsocketStream> {
    //     let url = self.inner.config.stream_base.clone();
    //     Ok(WebsocketStream::connect(self.clone(), url).await?)
    // }
}

impl RequestBuilder {
    pub fn uri(&self) -> String {
        self.request.get_uri().to_string()
    }

    pub fn query_arg<S: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: S,
        query: &T,
    ) -> KrakenResult<Self> {
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
            let uri = Uri::from_parts(parts).map_err(|e| KrakenError::other(format!("{:?}", e)))?;
            self.request = self.request.uri(uri);
        }

        Ok(self)
    }

    pub fn try_query_arg<S: AsRef<str>, T: Serialize>(
        self,
        name: S,
        query: &Option<T>,
    ) -> KrakenResult<Self> {
        match query {
            Some(val) => self.query_arg(name, val),
            None => Ok(self),
        }
    }

    pub fn auth_header(mut self) -> KrakenResult<Self> {
        self.request = self.request.header(
            "API-Key",
            HeaderValue::from_str(self.api_client.inner.config.api_key())?,
        );
        Ok(self)
    }

    pub fn request_body(mut self, payload: impl Serialize) -> KrakenResult<Self> {
        self.body = match &self.sign {
            Some((nonce,)) => serde_urlencoded::to_string(nonce.wrap(payload)),
            None => serde_urlencoded::to_string(payload),
        }?;
        Ok(self)
    }

    pub fn signed(mut self, nonce: Nonce) -> KrakenResult<Self> {
        self.sign = Some((nonce,));
        self.auth_header()
    }

    pub async fn send<V>(mut self) -> KrakenApiResult<V>
    where
        V: serde::de::DeserializeOwned,
    {
        self = self.sign().await?;
        self.request = self
            .request
            .content_type("application/x-www-form-urlencoded");
        log::debug!("{}  {}", self.request.get_method(), self.request.get_uri());
        log::debug!("{}", self.body);
        let tm = Instant::now();
        let mut res = self.request.send_body(self.body).await?;
        let d1 = tm.elapsed();
        let resp = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;
        log::debug!(
            "Request time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );
        log::debug!(
            "Response: {} «{}»",
            res.status(),
            String::from_utf8_lossy(&resp)
        );
        if let Err(err) = check_response(res) {
            // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
            Err(err)?
        };
        let answer: KrakenApiAnswer<V> = match serde_json::from_slice(&resp) {
            Ok(json) => json,
            Err(err) => {
                // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
                Err(err)?
            }
        };
        answer.into_api_result()
    }

    // pub async fn send_no_response(mut self) -> KrakenResult<()> {
    //     self = self.sign()?;
    //     log::debug!("{}  {}", self.request.get_method(), self.request.get_uri(),);
    //     let tm = Instant::now();
    //     let mut res = self.request.send().await?;
    //     let d1 = tm.elapsed();
    //     let resp = res.body().limit(16 * 1024 * 1024).await?;
    //     let d2 = tm.elapsed() - d1;
    //     log::debug!(
    //         "Request time elapsed:  {:0.1}ms + {:0.1}ms",
    //         d1.as_secs_f64() * 1000.0,
    //         d2.as_secs_f64() * 1000.0,
    //     );
    //     log::debug!(
    //         "Response: {} «{}»",
    //         res.status(),
    //         String::from_utf8_lossy(&resp)
    //     );
    //     if let Err(err) = check_response(res) {
    //         // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
    //         Err(err)?
    //     };
    //     Ok(())
    // }

    async fn sign(mut self) -> KrakenResult<Self> {
        if let Some((nonce,)) = self.sign {
            let path = self.request.get_uri().path();
            let signature = match self.signer {
                Signer::Cred(ref cred) => {
                    let decoded_secret = base64::decode(&cred.secret).map_err(|e| {
                        KrakenError::other(format!("Failed to deserialize key: {:?}", e))
                    })?;
                    sign(path, nonce, &self.body, &decoded_secret)
                }
                Signer::Hook(ref hook) => {
                    let nonce = nonce.value();
                    let data = Data {
                        nonce,
                        method: path,
                        params: Query::Url(&self.body),
                    };
                    hook.closure.as_ref().sign(data).await?
                }
            };
            self.request = self.request.header("API-Sign", signature);
        };
        Ok(self)
    }
}

/// Return base64 encoded api sign.
fn sign(path: &str, nonce: Nonce, body: &str, decoded_secret: &[u8]) -> String {
    // let payload = serde_urlencoded::to_string(payload).expect("serialize payload");
    let mut m256 = Sha256::new();
    m256.update(nonce.decimal());
    m256.update(body);
    let payload = m256.finalize();

    let mut m512 =
        Hmac::<Sha512>::new_varkey(decoded_secret).expect("HMAC can take key of any size");
    m512.update(path.as_bytes());
    m512.update(&payload);

    let res = m512.finalize().into_bytes();
    base64::encode(&res)
}

type AwcClientResponse = ClientResponse<Decoder<Payload<PayloadStream>>>;

fn check_response(res: AwcClientResponse) -> KrakenApiResult<AwcClientResponse> {
    use awc::http::StatusCode;

    // let used_rate_limits = UsedRateLimits::from_headers(res.headers());
    //
    // log::debug!("  used_rate_limits:  {:?}", used_rate_limits);

    match res.status() {
        StatusCode::OK => KrakenApiError::ok(res),
        StatusCode::INTERNAL_SERVER_ERROR => Err(ServiceError::ServerError)?,
        StatusCode::SERVICE_UNAVAILABLE => Err(ServiceError::ServiceUnavailable)?,
        // StatusCode::UNAUTHORIZED => Err(RequestError::Unauthorized)?,
        // StatusCode::BAD_REQUEST => {
        //     let error_json: BinanceContentError = response.json()?;
        //
        //     Err(ErrorKind::BinanceError(error_json.code, error_json.msg, response).into())
        // }
        s => Err(KrakenError::UnknownStatus(s))?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sign() {
        #[derive(Serialize)]
        struct Payload {
            ordertype: &'static str,
            pair: &'static str,
            price: &'static str,
            r#type: &'static str,
            volume: &'static str,
        }
        let encoded_secret = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
        let decoded_secret = base64::decode(&encoded_secret).unwrap();
        let nonce = Nonce::new(1616492376594_u64);
        // nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25
        let payload = Payload {
            ordertype: "limit",
            pair: "XBTUSD",
            price: "37500",
            r#type: "buy",
            volume: "1.25",
        };
        let path = "/0/private/AddOrder";
        let wrapped = nonce.wrap(&payload);
        let body = serde_urlencoded::to_string(wrapped).unwrap();
        let res = sign(path, nonce, &body, &decoded_secret);
        assert_eq!(
            res,
            "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ=="
        )
    }
}
