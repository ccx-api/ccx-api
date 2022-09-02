use std::sync::Arc;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::{BoxedPayloadStream, Payload};
use awc::http::Method;
use awc::http::StatusCode;
use serde::Serialize;

use ccx_api_lib::make_client;
use ccx_api_lib::Client;
use ccx_api_lib::ClientRequest;
use ccx_api_lib::ClientResponse;

use crate::client::BinancePaySigner;
use crate::error::BinanceError;
use crate::error::LibResult;
use crate::error::ServiceError;
use crate::Config;
use crate::LibError;
use crate::Time;

/// API client.
pub struct RestClient<S>
where
    S: BinancePaySigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestClient<S>
where
    S: BinancePaySigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner<S>
where
    S: BinancePaySigner,
{
    config: Config<S>,
}

pub struct RequestBuilder<S, J>
where
    S: BinancePaySigner,
    J: Serialize,
{
    api_client: RestClient<S>,
    request: ClientRequest,
    sign: Option<Time>,
    nonce: Option<String>,
    json: Option<J>,
}

impl<S> RestClient<S>
where
    S: BinancePaySigner,
{
    pub fn with_config(config: Config<S>) -> Self {
        let inner = Arc::new(ClientInner { config });
        RestClient { inner }
    }

    pub(super) fn client(&self) -> Client {
        make_client(false, self.inner.config.proxy.as_ref())
    }

    pub fn request<T: Serialize + Clone + 'static>(
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
            sign: None,
            nonce: None,
            json: None,
        })
    }

    pub fn post_json<T: Serialize + Clone + Sync + Send + 'static>(
        &self,
        endpoint: &str,
        value: T,
    ) -> LibResult<RequestBuilder<S, T>> {
        Ok(self.request(Method::POST, endpoint)?.json(value))
    }

    pub fn merchant_id(&self) -> u64 {
        self.inner.as_ref().config.merchant_id.value()
    }

    fn api_key(&self) -> &str {
        self.inner.as_ref().config.api_key()
    }
}

impl<S, J> RequestBuilder<S, J>
where
    S: BinancePaySigner,
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
            self.request = self.request.append_header((
                "BinancePay-Timestamp",
                &*time_window.timestamp().to_string(),
            ));
        }
        Ok(self)
    }

    pub fn nonce_header(mut self) -> LibResult<Self> {
        if let Some(nonce) = self.nonce.as_ref() {
            log::debug!("nonce_header :: {}", nonce);
            self.request = self
                .request
                .append_header(("BinancePay-Nonce", nonce.as_str()));
        }
        Ok(self)
    }

    pub fn api_key_header(mut self) -> LibResult<Self> {
        self.request = self
            .request
            .append_header(("BinancePay-Certificate-SN", self.api_client.api_key()));
        Ok(self)
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
        let signature = self
            .api_client
            .inner
            .config
            .signer()
            .sign_data(time.timestamp(), nonce, &json)
            .await?;
        let signature = signature.to_uppercase();
        self.request = self
            .request
            .append_header(("BinancePay-Signature", &*signature));
        Ok(self)
    }

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

type AwcClientResponse = ClientResponse<Decoder<Payload<BoxedPayloadStream>>>;

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
