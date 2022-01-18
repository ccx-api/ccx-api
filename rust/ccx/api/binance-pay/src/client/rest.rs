use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::{Payload, PayloadStream};
use awc::http::HeaderValue;
use awc::http::Method;
use awc::http::StatusCode;
use awc::Client;
use awc::ClientRequest;
use awc::ClientResponse;
use hmac::{Hmac, Mac, NewMac};
use serde::Serialize;
use sha2::Sha512;

use crate::client::SignBinancePay;
use crate::client::Signer;
use crate::client::Data;
use crate::error::BinanceError;
use crate::error::LibResult;
use crate::error::ServiceError;
use crate::Config;
use crate::LibError;
use crate::Time;
use crate::SignParams;

const CLIENT_TIMEOUT: u64 = 60;

/// API client.
#[derive(Default, Clone)]
pub struct RestClient {
    inner: Arc<ClientInner>,
}

#[derive(Default)]
struct ClientInner {
    config: Config,
}

pub struct RequestBuilder<J>
where
    J: Serialize,
{
    api_client: RestClient,
    request: ClientRequest,
    sign: Option<Time>,
    nonce: Option<String>,
    json: Option<J>,
}

impl RestClient {
    pub fn new() -> Self {
        RestClient::default()
    }

    pub fn with_config(config: Config) -> Self {
        let inner = Arc::new(ClientInner { config });
        RestClient { inner }
    }

    #[cfg(not(feature = "with_proxy"))]
    pub(super) fn client(&self) -> Client {
        let timeout = Duration::from_secs(CLIENT_TIMEOUT);
        let connector = awc::Connector::new().timeout(timeout).finish();
        Client::builder()
            .connector(connector)
            .timeout(timeout)
            .finish()
    }

    #[cfg(all(feature = "with_proxy"))]
    pub(super) fn client(&self) -> Client {
        use std::env::var;
        static CCX_BINANCE_API_PROXY_HOST: &str = "CCX_BINANCE_API_PROXY_HOST";
        static CCX_BINANCE_API_PROXY_PORT: &str = "CCX_BINANCE_API_PROXY_PORT";

        fn string_to_static_str(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }

        let host = var(CCX_BINANCE_API_PROXY_HOST)
            .expect("Error while getting CCX_BINANCE_API_PROXY_HOST");
        let port = var(CCX_BINANCE_API_PROXY_PORT)
            .expect("Error while getting CCX_BINANCE_API_PROXY_PORT");
        let proxy_addr = format!("{}:{}", host, port);
        let timeout = Duration::from_secs(CLIENT_TIMEOUT);
        awc::ClientBuilder::new()
            .connector(
                actix_web::client::Connector::new()
                    .connector(crate::client::connector::SocksConnector(
                        string_to_static_str(proxy_addr),
                    ))
                    .timeout(std::time::Duration::from_secs(60))
                    .finish(),
            )
            .timeout(timeout)
            .finish()
    }

    pub fn request<T: Serialize + Clone + 'static>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> LibResult<RequestBuilder<T>> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        Ok(RequestBuilder {
            api_client,
            request,
            sign: None,
            nonce: None,
            json: None,
        })
    }

    pub fn post_json<T: Serialize + Clone + Sync + Send + 'static>(
        &self,
        endpoint: &str,
        value: T,
    ) -> LibResult<RequestBuilder<T>> {
        Ok(self.request(Method::POST, endpoint)?.json(value))
    }

    pub fn merchant_id(&self) -> u64 {
        self.inner.as_ref().config.merchant_id.value()
    }

    fn api_key(&self) -> &str {
        self.inner.as_ref().config.api_key()
    }
}

impl<J> RequestBuilder<J>
where
    J: Serialize + Clone + Sync + Send + 'static,
{
    pub fn uri(&self) -> String {
        self.request.get_uri().to_string()
    }

    fn json(mut self, value: J) -> Self {
        self.json = Some(value);
        self
    }

    pub fn query_args<T: Serialize>(mut self, query: &T) -> LibResult<Self> {
        self.request = self.request.query(query)?;
        Ok(self)
    }

    pub fn timestamp_header(mut self) -> LibResult<Self> {
        if let Some(time_window) = self.sign {
            self.request = self.request.header(
                "BinancePay-Timestamp",
                HeaderValue::from_str(&time_window.timestamp().to_string())?,
            );
        }
        Ok(self)
    }

    pub fn nonce_header(mut self) -> LibResult<Self> {
        if let Some(nonce) = self.nonce.as_ref() {
            log::debug!("nonce_header :: {}", nonce);
            self.request = self
                .request
                .header("BinancePay-Nonce", HeaderValue::from_str(nonce.as_str())?);
        }
        Ok(self)
    }

    pub fn api_key_header(mut self) -> LibResult<Self> {
        self.request = self.request.header(
            "BinancePay-Certificate-SN",
            HeaderValue::from_str(&self.api_client.api_key())?,
        );
        Ok(self)
    }

    async fn sign_by_hook(
        time: Time,
        nonce: &str,
        params: &SignParams,
        closure: &dyn SignBinancePay,
    ) -> LibResult<String> {
        let data = Data {
            time: time.timestamp(),
            nonce,
            params,
        };
        closure
            .sign(data)
            .await
            .map_err(|e| e.into())
    }

    fn sign_by_cred<S>(time: &Time, nonce: &str, json: &S, secret: &[u8]) -> LibResult<String>
    where
        S: Serialize + 'static,
    {
        let timestamp = time.timestamp();
        let json = serde_json::to_string(json)?;
        log::debug!("payload_header json :: {}", json);
        let payload = format!("{}\n{}\n{}\n", timestamp, nonce, json);
        let signature = sign(&payload, secret);
        Ok(signature)
    }

    async fn payload_header(mut self) -> LibResult<Self> {
        let time = self
            .sign
            .ok_or_else(|| LibError::other("Time sign not found."))?;
        log::debug!("payload_header time :: {:?}", time);
        let nonce = self
            .nonce
            .as_ref()
            .ok_or_else(|| LibError::other("Nonce not found."))?;
        log::debug!("payload_header nonce :: {}", nonce);
        let json = self
            .json
            .clone()
            .ok_or_else(|| LibError::other("Body not found."))?;
        let signature = match self.api_client.inner.config.signer() {
            Signer::Cred(ref cred) => {
                Self::sign_by_cred(&time, nonce, &json, cred.secret.as_ref())?
            }
            Signer::Hook(ref hook) => {
                Self::sign_by_hook(time, nonce, &json, hook.closure.as_ref()).await?
            }
        };
        let signature = signature.to_uppercase();
        self.request = self
            .request
            .header("BinancePay-Signature", HeaderValue::from_str(&signature)?);
        Ok(self)
    }

    // async fn payload_header(mut self) -> LibResult<Self> {
    //     let timestamp = self
    //         .sign
    //         .ok_or_else(|| LibError::other("Time sign not found."))?
    //         .timestamp();
    //     log::debug!("payload_header timestamp :: {}", timestamp);
    //     let nonce = self
    //         .nonce
    //         .as_ref()
    //         .ok_or_else(|| LibError::other("Nonce not found."))?;
    //     log::debug!("payload_header nonce :: {}", nonce);
    //     let json = self
    //         .json
    //         .as_ref()
    //         .ok_or_else(|| LibError::other("Body not found."))?;
    //     let json = serde_json::to_string(json)?;
    //     log::debug!("payload_header json :: {}", json);
    //     let payload = format!("{}\n{}\n{}\n", timestamp, nonce, json);
    //     log::debug!("payload_header payload :: {}", payload);
    //     let signature = sign(&payload, self.api_client.api_secret().as_ref()).to_uppercase();
    //     self.request = self
    //         .request
    //         .header("BinancePay-Signature", HeaderValue::from_str(&signature)?);
    //     Ok(self)
    // }

    pub fn signed(mut self, time: impl Into<Time>) -> LibResult<Self> {
        self.sign = Some(time.into());
        self.timestamp_header()
    }

    pub fn nonce(mut self, nonce: String) -> LibResult<Self> {
        self.nonce = Some(nonce);
        self.nonce_header()
    }

    pub fn random_nonce(self) -> LibResult<Self> {
        let lchars = 'a'..'z';
        let uchars = 'A'..'Z';
        let chars = lchars
            .into_iter()
            .chain(uchars.into_iter())
            .collect::<String>();
        let charset_string = random_string::Charset::new(chars).unwrap();
        let res = random_string::generate(32, &charset_string);
        let nonce: String = res.to_string();
        self.nonce(nonce)
    }

    pub async fn send<V>(mut self) -> LibResult<V>
    where
        V: serde::de::DeserializeOwned,
    {
        self = self.api_key_header()?;
        self = self.payload_header().await?;
        let json = self.json.unwrap();

        log::debug!("{}  {}", self.request.get_method(), self.request.get_uri(),);
        let tm = Instant::now();
        // if true {
        //     log::debug!("send request :: {:?}", self.request);
        //     Err(LibError::other("STUB"))?
        // }
        log::debug!("send request :: {:?}", self.request);
        // let mut res = request.send().await?;
        let mut res = self.request.send_json(&json).await?;
        let d1 = tm.elapsed();
        let resp = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;
        log::debug!(
            "Request time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );
        // log::debug!("Response: {} «{}»", res.status(), String::from_utf8_lossy(&resp));
        log::debug!("Response: {} «{:#?}»", res.status(), resp);
        if let Err(err) = check_response(res, &resp) {
            // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
            return Err(err);
        };
        match serde_json::from_slice(&resp) {
            Ok(json) => Ok(json),
            Err(err) => {
                // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
                Err(LibError::Json(err))
            }
        }
    }
}

fn sign(query: &str, secret: &[u8]) -> String {
    log::debug!("sign query  :: {}", query);
    let mut mac = Hmac::<Sha512>::new_varkey(secret).expect("HMAC can take key of any size");
    mac.update(query.as_bytes());

    let res = mac.finalize().into_bytes();
    hex::encode(res)
}

type AwcClientResponse = ClientResponse<Decoder<Payload<PayloadStream>>>;

fn check_response(res: AwcClientResponse, resp: &[u8]) -> LibResult<AwcClientResponse> {
    match res.status() {
        StatusCode::OK => Ok(res),
        StatusCode::INTERNAL_SERVER_ERROR => Err(ServiceError::ServerError.into()),
        StatusCode::SERVICE_UNAVAILABLE => Err(ServiceError::ServiceUnavailable.into()),
        StatusCode::UNAUTHORIZED => {
            let error_json: BinanceError = serde_json::from_slice(resp)?;
            Err(error_json.into())
        }
        s => Err(LibError::UnknownStatus(s)),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_it_should_sign() {
        // let query = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&\
        //             recvWindow=5000&timestamp=1499827319559";
        // let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        // let res = sign(query, key.as_bytes());
        // assert_eq!(
        //     res,
        //     "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        // )
    }
}
