use std::future::Future;
use std::io;
use std::pin::Pin;

pub use ccx_api_lib::GateApiCred;
use sha2::Digest;
use smart_string::SmartString;
use thiserror::Error;

pub type ApiSignResult<'a> =
    Pin<Box<dyn Future<Output = Result<SmartString<128>, SignError>> + Send + 'a>>;
pub type ApiVerifyResult<'a> = Pin<Box<dyn Future<Output = Result<bool, SignError>> + Send + 'a>>;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Sign Server error: {0}")]
    ServerError(String),
}

pub trait GateSigner: Sync + Send {
    fn sign_api<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        request_method: &'b str,
        request_path: &'b str,
        request_query: &'b str,
        request_payload: &'b str,
        timestamp: &'b str,
    ) -> ApiSignResult<'a>;

    fn key(&self) -> &str;
}

impl GateSigner for GateApiCred {
    fn sign_api<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        request_method: &'b str,
        request_path: &'b str,
        request_query: &'b str,
        request_payload: &'b str,
        timestamp: &'b str,
    ) -> ApiSignResult<'a> {
        Box::pin(async move {
            Ok(sign(
                &self.secret,
                request_method,
                request_path,
                request_query,
                request_payload,
                timestamp,
            ))
        })
    }

    fn key(&self) -> &str {
        &self.key
    }
}

// Request Method + "\n" + Request URL + "\n" + Query String + "\n" + HexEncode(SHA512(Request Payload)) + "\n" + Timestamp
pub fn sign(
    secret: &str,
    request_method: &str,
    request_url: &str,
    request_query: &str,
    request_payload: &str,
    timestamp: &str,
) -> SmartString<128> {
    use hex::ToHex;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Sha512;

    let mut sha = Sha512::new();
    sha.update(request_payload.as_bytes());
    let hex_sha512_payload: SmartString<128> = sha.finalize().encode_hex();

    let mut mac =
        Hmac::<Sha512>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    mac.update(request_method.as_bytes());
    mac.update("\n".as_bytes());
    mac.update(request_url.as_bytes());
    mac.update("\n".as_bytes());
    mac.update(request_query.as_bytes());
    mac.update("\n".as_bytes());
    mac.update(hex_sha512_payload.as_bytes());
    mac.update("\n".as_bytes());
    mac.update(timestamp.as_bytes());

    mac.finalize().into_bytes().encode_hex()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [source](https://www.gate.io/docs/gatepay/common/en/#_2-4-2-signature-algorithm)
    const _REFERENCE_PYTHON_IMPLEMENTATION: &str = r#"

import hashlib
import hmac
def gen_sign(secret, timestamp, method, url, query_string=None, payload_string=None):
    m = hashlib.sha512()
    m.update((payload_string or "").encode('utf-8'))
    hashed_payload = m.hexdigest()
    s = '%s\n%s\n%s\n%s\n%s' % (method, url, query_string or "", hashed_payload, timestamp)
    return hmac.new(secret.encode('utf-8'), s.encode('utf-8'), hashlib.sha512).hexdigest()

gen_sign('123', '1700073707111', 'GET', '/api/v4/task', None, '{the_answer: 42}')    
    "#;

    #[test]
    fn test_sign() {
        let hex_digest = sign(
            "123",
            "GET",
            "/api/v4/task",
            "",
            "{the_answer: 42}",
            "1700073707111",
        );
        assert_eq!(
            hex_digest,
            "43299a924cf9663f5e6bfc6ab0c63eebc66d402c6dee61f0732b51b495206ff0\
             5f2fd19bef0bc00dcf87fb0c5c5f54abb0309a282734cdf29f5cd1230f64b6e7"
        );
    }
}
