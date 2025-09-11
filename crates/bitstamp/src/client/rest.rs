use std::sync::Arc;
use std::time::Instant;

use ::awc::http::Method;
use ::awc::http::StatusCode;
use actix_http::BoxedPayloadStream;
use actix_http::Payload;
use actix_http::Uri;
use actix_http::encoding::Decoder;
use ccx_api_lib::Client;
use ccx_api_lib::ClientRequest;
use ccx_api_lib::ClientResponse;
use ccx_api_lib::make_client;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

// use crate::client::limits::UsedRateLimits;
use crate::client::WebsocketStream;
use crate::client::*;
use crate::error::*;
// use crate::proto::TimeWindow;

/// API client.
pub struct RestClient<S>
where
    S: BitstampSigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestClient<S>
where
    S: BitstampSigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
struct ClientInner<S>
where
    S: BitstampSigner,
{
    config: Config<S>,
}

pub struct RequestBuilder<S>
where
    S: BitstampSigner,
{
    api_client: RestClient<S>,
    request: ClientRequest,
    sign: Option<(Nonce, u64)>,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct BitstampApiAnswer<T> {
    result: T,
    message: String,
}

impl<S> RestClient<S>
where
    S: BitstampSigner,
{
    pub fn new(config: Config<S>) -> Self {
        let inner = Arc::new(ClientInner { config });
        RestClient { inner }
    }

    pub(super) fn client(&self) -> Client {
        make_client(false, self.inner.config.proxy.as_ref())
    }

    pub fn request(&self, method: Method, endpoint: &str) -> BitstampResult<RequestBuilder<S>> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        let request = self.client().request(method, url.as_str());
        Ok(RequestBuilder {
            api_client,
            request,
            sign: None,
            body: String::new(),
        })
    }

    pub fn get(&self, endpoint: &str) -> BitstampResult<RequestBuilder<S>> {
        self.request(Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: &str) -> BitstampResult<RequestBuilder<S>> {
        self.request(Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: &str) -> BitstampResult<RequestBuilder<S>> {
        self.request(Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: &str) -> BitstampResult<RequestBuilder<S>> {
        self.request(Method::DELETE, endpoint)
    }

    pub async fn web_socket(&self) -> BitstampResult<WebsocketStream> {
        let url = self.inner.config.stream_base.clone();
        Ok(WebsocketStream::connect(self.clone(), url).await?)
    }
}

impl<S> RequestBuilder<S>
where
    S: BitstampSigner,
{
    pub fn uri(&self) -> String {
        self.request.get_uri().to_string()
    }

    pub fn query_arg<Name: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: Name,
        query: &T,
    ) -> BitstampResult<Self> {
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
                Uri::from_parts(parts).map_err(|e| BitstampError::other(format!("{:?}", e)))?;
            self.request = self.request.uri(uri);
        }

        Ok(self)
    }

    pub fn try_query_arg<Name: AsRef<str>, T: Serialize>(
        self,
        name: Name,
        query: &Option<T>,
    ) -> BitstampResult<Self> {
        match query {
            Some(val) => self.query_arg(name, val),
            None => Ok(self),
        }
    }

    pub fn request_body(mut self, payload: impl Serialize) -> BitstampResult<Self> {
        self.body = serde_urlencoded::to_string(&payload)?;

        if !self.body.is_empty() {
            self.request = self
                .request
                .content_type("application/x-www-form-urlencoded");
        }

        Ok(self)
    }

    pub fn signed(mut self, nonce: Nonce, timestamp: u64) -> BitstampResult<Self> {
        self.sign = Some((nonce, timestamp));
        Ok(self)
    }

    pub fn signed_now(self) -> BitstampResult<Self> {
        let nonce = Nonce::new();
        let timestamp = Utc::now().timestamp_millis() as u64;
        self.signed(nonce, timestamp)
    }

    pub async fn send<V>(mut self) -> BitstampApiResult<V>
    where
        V: DeserializeOwned,
    {
        let request_id = Uuid::new_v4();
        self = self.sign().await?;

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

    async fn sign(mut self) -> BitstampResult<Self> {
        if let Some((nonce, timestamp)) = self.sign {
            let req_host = self
                .request
                .get_uri()
                .host()
                .ok_or_else(|| BitstampApiError::lib_error(&"Missing Host"))?;

            let req_content_type = self
                .request
                .headers()
                .get(actix_http::header::CONTENT_TYPE)
                .map(|h| h.to_str().ok())
                .unwrap_or_default()
                .unwrap_or_default();

            let req_path = self.request.get_uri().path();
            let req_query = self.request.get_uri().query().unwrap_or_default();
            let req_method = self.request.get_method();

            // path should be like /api/v2/balance/
            let api_version = req_path.split('/').nth(2).unwrap_or_default().to_string();
            let auth_key = format!("BITSTAMP {}", self.api_client.inner.config.api_key());

            if *req_method == Method::GET {
                self.body = String::new();
            }

            let signature = self
                .api_client
                .inner
                .config
                .signer()
                .sign_data(
                    nonce,
                    timestamp,
                    req_method.as_str(),
                    req_host,
                    req_path,
                    req_query,
                    req_content_type,
                    &api_version,
                    &self.body,
                )
                .await?;

            self.request = self
                .request
                .append_header(("X-Auth", auth_key))
                .append_header(("X-Auth-Signature", signature))
                .append_header(("X-Auth-Nonce", nonce.to_string()))
                .append_header(("X-Auth-Timestamp", timestamp.to_string()))
                .append_header(("X-Auth-Version", api_version));
        };

        self.request = self
            .request
            .append_header(("Accept", "application/json"))
            .append_header(("User-Agent", "ccx-api/0.4 (lib; Rust)"));

        Ok(self)
    }
}

/// Return base64 encoded api sign.

type AwcClientResponse = ClientResponse<Decoder<Payload<BoxedPayloadStream>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApiErrorMessage {
    reason: String,
}

fn check_response(res: AwcClientResponse) -> BitstampApiResult<AwcClientResponse> {
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

fn from_response<V: DeserializeOwned>(code: StatusCode, body: &[u8]) -> BitstampApiResult<V> {
    #[derive(Deserialize)]
    struct StatusWrapper<'a> {
        status: &'a str,
    }
    let wrapper: StatusWrapper =
        serde_json::from_slice(body).unwrap_or(StatusWrapper { status: "ok" });

    match () {
        _ if code.is_success() && wrapper.status != "error" => match serde_json::from_slice(body) {
            Ok(result) => Ok(result),
            Err(err) => Err(err)?,
        },
        _ => {
            let message = match serde_json::from_slice(body) {
                Ok(ApiErrorMessage { reason }) => reason,
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
