use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use ::reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

use crate::client::*;
use crate::error::*;
use crate::Uuid;

/// API client.
pub struct RestTradeClient<S>
where
    S: CoinbaseTradeSigner,
{
    inner: Arc<ClientInner<S>>,
}

impl<S> Clone for RestTradeClient<S>
where
    S: CoinbaseTradeSigner,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner<S>
where
    S: CoinbaseTradeSigner,
{
    config: TradeConfig<S>,
    client: Client,
}

pub struct TradeRequestBuilder<S>
where
    S: CoinbaseTradeSigner,
{
    api_client: RestTradeClient<S>,
    url: Url,
    method: Method,
    query_params: Vec<(String, String)>,
    headers: HashMap<String, String>,
    sign: Option<(u32,)>,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct CoinbaseApiAnswer<T> {
    result: T,
    message: String,
}

impl<S> RestTradeClient<S>
where
    S: CoinbaseTradeSigner,
{
    pub fn new(config: TradeConfig<S>) -> Self {
        let client = Self::create_client(config.proxy.as_ref());
        let inner = Arc::new(ClientInner { config, client });
        RestTradeClient { inner }
    }

    fn create_client(proxy: Option<&crate::client::Proxy>) -> Client {
        let mut builder = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(60));
            
        if let Some(proxy) = proxy {
            let proxy_url = format!("socks5://{}", proxy.addr());
            if let Ok(reqwest_proxy) = ::reqwest::Proxy::all(&proxy_url) {
                builder = builder.proxy(reqwest_proxy);
            }
        }
        
        builder.build().expect("Failed to create HTTP client")
    }

    pub fn client(&self) -> &Client {
        &self.inner.client
    }

    pub fn request(
        &self,
        method: Method,
        endpoint: &str,
    ) -> CoinbaseResult<TradeRequestBuilder<S>> {
        let url = self.inner.config.api_base.join(endpoint)?;
        log::debug!("Requesting: {}", url.as_str());
        let api_client = self.clone();
        Ok(TradeRequestBuilder {
            api_client,
            url,
            method,
            query_params: Vec::new(),
            headers: HashMap::new(),
            sign: None,
            body: String::new(),
        })
    }

    pub fn get(&self, endpoint: &str) -> CoinbaseResult<TradeRequestBuilder<S>> {
        self.request(Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: &str) -> CoinbaseResult<TradeRequestBuilder<S>> {
        self.request(Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: &str) -> CoinbaseResult<TradeRequestBuilder<S>> {
        self.request(Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: &str) -> CoinbaseResult<TradeRequestBuilder<S>> {
        self.request(Method::DELETE, endpoint)
    }
}

impl<S> TradeRequestBuilder<S>
where
    S: CoinbaseTradeSigner,
{
    pub fn uri(&self) -> String {
        self.url.to_string()
    }

    pub fn query_arg<Name: AsRef<str>, T: Serialize + ?Sized>(
        mut self,
        name: Name,
        query: &T,
    ) -> CoinbaseResult<Self> {
        let serialized = serde_json::to_string(query)?;
        self.query_params
            .push((name.as_ref().to_string(), serialized));
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
        self.headers.insert(
            "API-Key".to_string(),
            self.api_client.inner.config.api_key().to_string(),
        );
        Ok(self)
    }

    pub fn request_body(mut self, payload: impl Serialize) -> CoinbaseResult<Self> {
        self.body = serde_json::to_string(&payload)?;
        Ok(self)
    }

    pub fn signed(mut self, timestamp: u32) -> CoinbaseResult<Self> {
        self.sign = Some((timestamp,));
        Ok(self)
    }

    pub async fn send<V>(mut self) -> CoinbaseApiResult<V>
    where
        V: DeserializeOwned,
    {
        let request_id = Uuid::new_v4();
        self = self.sign().await?;

        // Build the request using reqwest API directly
        let mut request = self.api_client.inner.client.request(self.method.clone(), self.url.clone());

        // Add query parameters
        if !self.query_params.is_empty() {
            request = request.query(&self.query_params);
        }

        // Add headers
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        request = request.header("content-type", "application/json");

        log::debug!("[{request_id}]  Request: {} {}", self.method, self.url);

        if cfg!(feature = "debug_headers") {
            for (name, value) in &self.headers {
                let value = value;
                log::debug!("[{request_id}]  Request header: {name}: {value}",);
            }
        }
        log::debug!("[{request_id}]  Request body: {:?}", self.body);

        let tm = Instant::now();
        let res = if self.body.is_empty() {
            request.send().await?
        } else {
            request.body(self.body).send().await?
        };
        let d1 = tm.elapsed();

        let code = res.status();
        log::debug!("[{request_id}]  Response status: {code}");

        if cfg!(feature = "debug_headers") {
            for (name, value) in res.headers().iter() {
                let value = value.to_str().unwrap_or("---");
                log::debug!("[{request_id}]  Response header: {name}: {value}",);
            }
        }

        let resp = res.bytes().await?;
        let d2 = tm.elapsed() - d1;

        log::debug!(
            "[{request_id}]  Time elapsed:  {:0.1}ms + {:0.1}ms",
            d1.as_secs_f64() * 1000.0,
            d2.as_secs_f64() * 1000.0,
        );

        log::debug!(
            "[{request_id}]  Response body: {:?}",
            String::from_utf8_lossy(&resp)
        );

        check_response(code)?;
        from_response(code, &resp)
    }

    async fn sign(mut self) -> CoinbaseResult<Self> {
        if let Some((timestamp,)) = self.sign {
            let path_and_query = format!(
                "{}{}",
                self.url.path(),
                self.url
                    .query()
                    .map(|q| format!("?{}", q))
                    .unwrap_or_default()
            );

            if self.method == Method::GET {
                self.body = String::new();
            }

            let signature = self
                .api_client
                .inner
                .config
                .signer()
                .sign_data(timestamp, self.method.as_str(), &path_and_query, &self.body)
                .await?;

            self.headers
                .insert("Authorization".to_string(), format!("Bearer {}", signature));
            self.headers.insert(
                "CB-ACCESS-KEY".to_string(),
                self.api_client.inner.config.api_key().to_string(),
            );
            self.headers
                .insert("CB-ACCESS-TIMESTAMP".to_string(), timestamp.to_string());
        };

        self.headers
            .insert("Accept".to_string(), "application/json".to_string());
        self.headers.insert(
            "User-Agent".to_string(),
            "ccx-api/0.4 (lib; Rust)".to_string(),
        );

        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApiErrorMessage {
    message: String,
}

fn check_response(status: StatusCode) -> CoinbaseApiResult<()> {
    match status {
        StatusCode::OK => Ok(()),
        StatusCode::TOO_MANY_REQUESTS => Err(ApiServiceError::RateLimitExceeded)?,
        StatusCode::INTERNAL_SERVER_ERROR => Err(ApiServiceError::ServerError)?,
        StatusCode::BAD_GATEWAY => Err(ApiServiceError::ServiceUnavailable)?,
        StatusCode::SERVICE_UNAVAILABLE => Err(ApiServiceError::ServiceUnavailable)?,
        StatusCode::GATEWAY_TIMEOUT => Err(ApiServiceError::ServiceUnavailable)?,
        _ if status.is_success() => Ok(()),
        _ => Ok(()), // Let from_response handle other error codes
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
