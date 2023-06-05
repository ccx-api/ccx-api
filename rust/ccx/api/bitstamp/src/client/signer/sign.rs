use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;

use super::BitstampSigner;
use super::Nonce;
use crate::BitstampResult;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = BitstampResult<String>> + Send + 'a>>;

impl BitstampSigner for ApiCred {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        timestamp: u64,
        http_method: &'b str,
        http_host: &'b str,
        http_path: &'b str,
        http_query: &'b str,
        content_type: &'b str,
        version: &'b str,
        body: &'b str,
    ) -> SignResult<'a> {
        Box::pin(async move {
            Ok(sign(
                &self.key,
                &self.secret,
                nonce,
                timestamp,
                http_method,
                http_host,
                http_path,
                http_query,
                content_type,
                version,
                body,
            ))
        })
    }

    fn api_key(&self) -> &str {
        self.key.as_str()
    }
}

#[allow(clippy::too_many_arguments)]
fn sign(
    key: &str,
    secret: &str,
    nonce: Nonce,
    timestamp: u64,
    http_method: &str,
    http_host: &str,
    http_path: &str,
    http_query: &str,
    content_type: &str,
    version: &str,
    body: &str,
) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    mac.update(format!("BITSTAMP {}", key).as_bytes());
    mac.update(http_method.as_bytes());
    mac.update(http_host.as_bytes());
    mac.update(http_path.as_bytes());
    mac.update(http_query.as_bytes());
    mac.update(content_type.as_bytes());
    mac.update(nonce.to_string().as_bytes());
    mac.update(timestamp.to_string().as_bytes());
    mac.update(version.as_bytes());
    mac.update(body.as_bytes());

    let payload = mac.finalize().into_bytes();
    hex::encode(payload)
}
