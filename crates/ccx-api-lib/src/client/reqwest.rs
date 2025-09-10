use std::time::Duration;

pub use reqwest::Error as SendRequestError;
pub use reqwest::Method;
pub use reqwest::StatusCode;

pub use crate::Proxy;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

// Wrapper around reqwest::Client to provide awc-compatible interface
#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn request(&self, method: Method, url: &str) -> ClientRequest {
        let url_parsed = url::Url::parse(url).expect("Invalid URL");
        ClientRequest {
            inner: self.inner.request(method.clone(), url_parsed.clone()),
            url: url_parsed,
            method,
            headers: std::collections::HashMap::new(),
        }
    }
}

// Wrapper around reqwest::RequestBuilder to provide awc-compatible interface
pub struct ClientRequest {
    inner: reqwest::RequestBuilder,
    url: url::Url,
    method: Method,
    headers: std::collections::HashMap<String, String>,
}

impl ClientRequest {
    pub fn url(&self) -> &url::Url {
        &self.url
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.headers
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
        Self {
            inner: self.inner.header(key.as_ref(), value.as_ref()),
            url: self.url,
            method: self.method,
            headers: self.headers,
        }
    }

    pub fn query<T: serde::Serialize + ?Sized>(self, query: &T) -> Self {
        Self {
            inner: self.inner.query(query),
            url: self.url,
            method: self.method,
            headers: self.headers,
        }
    }

    pub fn body<T: Into<reqwest::Body>>(self, body: T) -> Self {
        Self {
            inner: self.inner.body(body),
            url: self.url,
            method: self.method,
            headers: self.headers,
        }
    }

    pub fn headers(&self) -> &std::collections::HashMap<String, String> {
        &self.headers
    }

    pub async fn send(self) -> Result<ClientResponse, reqwest::Error> {
        let response = self.inner.send().await?;
        Ok(ClientResponse { inner: response })
    }
}

// Wrapper around reqwest::Response to provide awc-compatible interface
pub struct ClientResponse {
    inner: reqwest::Response,
}

impl ClientResponse {
    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        self.inner.headers()
    }

    pub async fn bytes(self) -> Result<bytes::Bytes, reqwest::Error> {
        self.inner.bytes().await
    }

    pub async fn text(self) -> Result<String, reqwest::Error> {
        self.inner.text().await
    }
}

pub fn make_client(_h1_only: bool, proxy: Option<&Proxy>) -> Client {
    match proxy {
        Some(proxy) => client_with_proxy(proxy),
        None => client_without_proxy(),
    }
}

pub fn client_without_proxy() -> Client {
    let inner = reqwest::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(CLIENT_TIMEOUT)
        .build()
        .expect("Failed to create HTTP client");

    Client { inner }
}

pub fn client_with_proxy(proxy: &Proxy) -> Client {
    let proxy_url = format!("socks5://{}", proxy.addr());
    let reqwest_proxy = reqwest::Proxy::all(&proxy_url).expect("Failed to create proxy");

    let inner = reqwest::Client::builder()
        .proxy(reqwest_proxy)
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(CLIENT_TIMEOUT)
        .build()
        .expect("Failed to create HTTP client with proxy");

    Client { inner }
}
