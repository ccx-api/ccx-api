use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::{Payload, PayloadStream, Method, Uri};
use awc::Connector;
use awc::{Client, ClientRequest, ClientResponse};
use serde::Serialize;

use ccx_api_lib::SocksConnector;

use super::*;
use crate::client::limits::UsedRateLimits;
use crate::client::{WebsocketClient, WebsocketStream};
use crate::error::*;
use crate::proto::TimeWindow;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

/// API client.
pub struct RestClient<S>
where
    S: BinanceSigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestClient<S>
where
    S: BinanceSigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner<S>
where
    S: BinanceSigner,
{
    config: Config<S>,
}

pub struct RequestBuilder<S>
where
    S: BinanceSigner,
{
    api_client: RestClient<S>,
    request: ClientRequest,
    sign: Option<TimeWindow>,
}

impl<S> RestClient<S>
where
    S: BinanceSigner,
{
    pub fn new(config: Config<S>) -> Self {
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

    pub(crate) fn as_dyn(&self) -> RestClient<std::sync::Arc<dyn BinanceSigner>> {
        let config = self.inner.config.as_dyn();
        let inner = Arc::new(ClientInner { config });
        RestClient { inner }
    }

    pub(super) fn client(&self) -> Client {
        self.client_(false)
    }

    pub(super) fn client_h1(&self) -> Client {
        self.client_(true)
    }

    fn client_config(h1_only: bool) -> Arc<rustls::ClientConfig> {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

        let mut cfg = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        if h1_only {
            cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
        }
        Arc::new(cfg)
    }

    fn client_without_proxy(&self, cfg: Arc<rustls::ClientConfig>) -> Client {
        let connector = Connector::new().rustls(cfg).timeout(CONNECT_TIMEOUT);
        Client::builder()
            .connector(connector)
            .timeout(CLIENT_TIMEOUT)
            .finish()
    }

    fn client_with_proxy(&self, _cfg: Arc<rustls::ClientConfig>, _proxy: &Proxy) -> Client {
        // let connector = Connector::new()
        //     .rustls(cfg)
        //     .connector(SocksConnector::new(proxy.addr()))
        //     .timeout(CONNECT_TIMEOUT);
        // Client::builder()
        //     .connector(connector)
        //     .timeout(CLIENT_TIMEOUT)
        //     .finish()
        todo!("FIX client_with_proxy")
    }

    pub fn request(&self, method: Method, endpoint: &str) -> BinanceResult<RequestBuilder<S>> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        Ok(RequestBuilder {
            api_client,
            request,
            sign: None,
        })
    }

    pub fn get(&self, endpoint: &str) -> BinanceResult<RequestBuilder<S>> {
        self.request(Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: &str) -> BinanceResult<RequestBuilder<S>> {
        self.request(Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: &str) -> BinanceResult<RequestBuilder<S>> {
        self.request(Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: &str) -> BinanceResult<RequestBuilder<S>> {
        self.request(Method::DELETE, endpoint)
    }

    pub async fn web_socket(&self) -> BinanceResult<WebsocketClient> {
        let url = self.inner.config.stream_base.clone();
        Ok(WebsocketClient::connect(self.clone(), url).await?)
    }

    pub async fn web_socket2(&self) -> BinanceResult<WebsocketStream> {
        let url = self.inner.config.stream_base.clone();
        Ok(WebsocketStream::connect(self.clone(), url).await?)
    }
}

impl<S> RequestBuilder<S>
where
    S: BinanceSigner,
{
    pub fn uri(&self) -> String {
        self.request.get_uri().to_string()
    }

    pub fn query_args<T: Serialize>(mut self, query: &T) -> BinanceResult<Self> {
        self.request = self.request.query(query)?;
        Ok(self)
    }

    pub fn query_arg<Name: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: Name,
        query: &T,
    ) -> BinanceResult<Self> {
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
            let uri =
                Uri::from_parts(parts).map_err(|e| BinanceError::other(format!("{:?}", e)))?;
            self.request = self.request.uri(uri);
        }

        Ok(self)
    }

    pub fn try_query_arg<Name: AsRef<str>, T: Serialize>(
        self,
        name: Name,
        query: &Option<T>,
    ) -> BinanceResult<Self> {
        match query {
            Some(val) => self.query_arg(name, val),
            None => Ok(self),
        }
    }

    pub fn auth_header(mut self) -> BinanceResult<Self> {
        self.request = self.request.append_header((
            "X-MBX-APIKEY",
            self.api_client.inner.config.api_key(),
        ));
        Ok(self)
    }

    pub fn signed(mut self, time_window: impl Into<TimeWindow>) -> BinanceResult<Self> {
        self.sign = Some(time_window.into());
        self.auth_header()
    }

    pub async fn send<V>(mut self) -> BinanceResult<V>
    where
        V: serde::de::DeserializeOwned,
    {
        self = if let Some(sign) = self.sign.clone() {
            self = self.query_arg("timestamp", &sign.timestamp())?;
            let recv_window = sign.recv_window();
            if !recv_window.is_default() {
                self = self.query_arg("recvWindow", &*recv_window)?;
            }
            self.sign().await?
        } else {
            self
        };
        log::debug!("{}  {}", self.request.get_method(), self.request.get_uri(),);
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
        log::debug!(
            "Response: {} «{}»",
            res.status(),
            String::from_utf8_lossy(&resp)
        );
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

    pub async fn send_no_response(mut self) -> BinanceResult<()> {
        self = if let Some(sign) = self.sign.clone() {
            self = self.query_arg("timestamp", &sign.timestamp())?;
            let recv_window = sign.recv_window();
            if !recv_window.is_default() {
                self = self.query_arg("recvWindow", &*recv_window)?;
            }
            self.sign().await?
        } else {
            self
        };
        log::debug!("{}  {}", self.request.get_method(), self.request.get_uri(),);
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
        log::debug!(
            "Response: {} «{}»",
            res.status(),
            String::from_utf8_lossy(&resp)
        );
        if let Err(err) = check_response(res) {
            // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
            Err(err)?
        };
        Ok(())
    }

    async fn sign(self) -> BinanceResult<Self> {
        let query = self.request.get_uri().query().unwrap_or("");
        let signature = self
            .api_client
            .inner
            .config
            .signer()
            .sign_data(query)
            .await?;
        self.query_arg("signature", &signature)
    }
}

type AwcClientResponse = ClientResponse<Decoder<Payload<PayloadStream>>>;

fn check_response(res: AwcClientResponse) -> BinanceResult<AwcClientResponse> {
    use awc::http::StatusCode;

    let used_rate_limits = UsedRateLimits::from_headers(res.headers());

    log::debug!("  used_rate_limits:  {:?}", used_rate_limits);

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
        s => Err(BinanceError::UnknownStatus(s))?,
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_should_sign() {
//         let query = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&\
//                     recvWindow=5000&timestamp=1499827319559";
//         let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
//         let res = sign(query, key.as_bytes());
//         assert_eq!(
//             res,
//             "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
//         )
//     }
// }
