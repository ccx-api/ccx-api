use std::sync::Arc;
use std::time::Instant;

use actix_http::encoding::Decoder;
use actix_http::BoxedPayloadStream;
use actix_http::Payload;
use actix_http::Uri;
use awc::http::Method;
use awc::http::StatusCode;
use ccx_api_lib::make_client;
use ccx_api_lib::Client;
use ccx_api_lib::ClientRequest;
use ccx_api_lib::ClientResponse;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::client::*;
// use crate::client::limits::UsedRateLimits;
// use crate::client::WebsocketStream;
use crate::error::*;
// use crate::proto::TimeWindow;

/// API client.
pub struct RestPrimeClient<S>
where
    S: CoinbasePrimeSigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestPrimeClient<S>
where
    S: CoinbasePrimeSigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner<S>
where
    S: CoinbasePrimeSigner,
{
    config: PrimeConfig<S>,
}

pub struct PrimeRequestBuilder<S>
where
    S: CoinbasePrimeSigner,
{
    api_client: RestPrimeClient<S>,
    request: ClientRequest,
    sign: Option<(u32,)>,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct CoinbaseApiAnswer<T> {
    result: T,
    message: String,
}

impl<S> RestPrimeClient<S>
where
    S: CoinbasePrimeSigner,
{
    pub fn new(config: PrimeConfig<S>) -> Self {
        let inner = Arc::new(ClientInner { config });
        RestPrimeClient { inner }
    }

    pub(super) fn client(&self) -> Client {
        make_client(false, self.inner.config.proxy.as_ref())
    }

    pub fn request(
        &self,
        method: Method,
        endpoint: &str,
    ) -> CoinbaseResult<PrimeRequestBuilder<S>> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        Ok(PrimeRequestBuilder {
            api_client,
            request,
            sign: None,
            body: String::new(),
        })
    }

    pub fn get(&self, endpoint: &str) -> CoinbaseResult<PrimeRequestBuilder<S>> {
        self.request(Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: &str) -> CoinbaseResult<PrimeRequestBuilder<S>> {
        self.request(Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: &str) -> CoinbaseResult<PrimeRequestBuilder<S>> {
        self.request(Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: &str) -> CoinbaseResult<PrimeRequestBuilder<S>> {
        self.request(Method::DELETE, endpoint)
    }
    //
    // pub async fn web_socket(&self) -> CoinbaseResult<WebsocketStream> {
    //     let url = self.inner.config.stream_base.clone();
    //     Ok(WebsocketStream::connect(self.clone(), url).await?)
    // }
}

impl<S> PrimeRequestBuilder<S>
where
    S: CoinbasePrimeSigner,
{
    pub fn uri(&self) -> String {
        self.request.get_uri().to_string()
    }

    pub fn query_arg<Name: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: Name,
        query: &T,
    ) -> CoinbaseResult<Self> {
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
            buf.push_str(&serde_urlencoded::to_string([(name.as_ref(), query)])?);
            parts.path_and_query = buf.parse().ok();
            let uri =
                Uri::from_parts(parts).map_err(|e| CoinbaseError::other(format!("{:?}", e)))?;
            self.request = self.request.uri(uri);
        }

        Ok(self)
    }

    pub fn try_query_arg<Name: AsRef<str>, T: Serialize>(
        self,
        name: Name,
        query: &Option<T>,
    ) -> CoinbaseResult<Self> {
        match query {
            Some(val) => self.query_arg(name, val),
            None => Ok(self),
        }
    }

    pub fn auth_header(mut self) -> CoinbaseResult<Self> {
        self.request = self
            .request
            .append_header(("API-Key", self.api_client.inner.config.api_key()));
        Ok(self)
    }

    pub fn request_body(mut self, payload: impl Serialize) -> CoinbaseResult<Self> {
        self.body = serde_json::to_string(&payload)?;
        Ok(self)
    }

    pub fn signed(mut self, timestamp: u32) -> CoinbaseResult<Self> {
        self.sign = Some((timestamp,));
        self.auth_header()
    }

    pub async fn send<V>(mut self) -> CoinbaseApiResult<V>
    where
        V: DeserializeOwned,
    {
        let request_id = Uuid::new_v4();
        self = self.sign().await?;

        self.request = self.request.content_type("application/json");
        log::debug!(
            "[{request_id}]  Request: {} {}",
            self.request.get_method(),
            self.request.get_uri()
        );
        log::debug!("[{request_id}]  Request body: {:?}", self.body);

        let tm = Instant::now();
        let mut res = self.request.send_body(self.body).await?;
        let d1 = tm.elapsed();
        let resp = res.body().limit(16 * 1024 * 1024).await?;
        let d2 = tm.elapsed() - d1;
        log::debug!(
            "[{request_id}]  Time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );

        let code = res.status();
        log::debug!(
            "[{request_id}]  Response: {} «{}»",
            code,
            String::from_utf8_lossy(&resp)
        );

        if let Err(err) = check_response(res) {
            // log::debug!("Response: {}", String::from_utf8_lossy(&resp));
            Err(err)?
        };

        from_response(code, &resp)
    }

    // pub async fn send_no_response(mut self) -> CoinbaseResult<()> {
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

    async fn sign(mut self) -> CoinbaseResult<Self> {
        if let Some((timestamp,)) = self.sign {
            let url_path = self.request.get_uri().path();
            let method = self.request.get_method();

            if *method == Method::GET {
                self.body = String::new();
            }

            let signature = self
                .api_client
                .inner
                .config
                .signer()
                .sign_data(timestamp, method.as_str(), url_path, &self.body)
                .await?;

            self.request = self
                .request
                .append_header(("X-CB-ACCESS-SIGNATURE", signature))
                .append_header(("X-CB-ACCESS-timestamp", timestamp))
                .append_header(("X-CB-ACCESS-KEY", self.api_client.inner.config.api_key()))
                .append_header((
                    "X-CB-ACCESS-PASSPHRASE",
                    self.api_client.inner.config.api_passphrase(),
                ));
        };

        self.request = self.request.append_header(("Accept", "application/json"));

        Ok(self)
    }
}

/// Return base64 encoded api sign.

type AwcClientResponse = ClientResponse<Decoder<Payload<BoxedPayloadStream>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApiErrorMessage {
    message: String,
}

fn check_response(res: AwcClientResponse) -> CoinbaseApiResult<AwcClientResponse> {
    // let used_rate_limits = UsedRateLimits::from_headers(res.headers());
    //
    // log::debug!("  used_rate_limits:  {:?}", used_rate_limits);

    match res.status() {
        StatusCode::OK => Ok(res),
        // TODO check for rate limit error
        StatusCode::TOO_MANY_REQUESTS => Err(ApiServiceError::RateLimitExceeded)?,
        StatusCode::INTERNAL_SERVER_ERROR => Err(ApiServiceError::ServerError)?,
        StatusCode::BAD_GATEWAY => Err(ApiServiceError::ServiceUnavailable)?,
        StatusCode::SERVICE_UNAVAILABLE => Err(ApiServiceError::ServiceUnavailable)?,
        StatusCode::GATEWAY_TIMEOUT => Err(ApiServiceError::ServiceUnavailable)?,
        _ => Ok(res),
    }
}

fn from_response<V: DeserializeOwned>(code: StatusCode, body: &[u8]) -> CoinbaseApiResult<V> {
    match code {
        _ if code.is_success() => match serde_json::from_slice(body) {
            Ok(result) => Ok(result),
            Err(err) => Err(err)?,
        },
        _ => {
            let message = match serde_json::from_slice(body) {
                Ok(ApiErrorMessage { message }) => message,
                Err(_err) => String::from_utf8_lossy(body).to_string(),
            };
            let kind = match code {
                StatusCode::UNAUTHORIZED => ApiErrorKind::Unauthorized,
                _ => ApiErrorKind::Unrecognized,
            };
            Err(LibError::ApiError((kind, code, message).into()))?
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_should_sign() {
//         #[derive(Serialize)]
//         struct Payload {
//             ordertype: &'static str,
//             pair: &'static str,
//             price: &'static str,
//             r#type: &'static str,
//             volume: &'static str,
//         }
//         let encoded_secret = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
//         let decoded_secret = base64::decode(&encoded_secret).unwrap();
//         let nonce = Nonce::new(1616492376594_u64);
//         // nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25
//         let payload = Payload {
//             ordertype: "limit",
//             pair: "XBTUSD",
//             price: "37500",
//             r#type: "buy",
//             volume: "1.25",
//         };
//         let path = "/0/private/AddOrder";
//         let wrapped = nonce.wrap(&payload);
//         let body = serde_urlencoded::to_string(wrapped).unwrap();
//         let res = sign(path, nonce, &body, &decoded_secret);
//         assert_eq!(
//             res,
//             "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ=="
//         )
//     }
// }
