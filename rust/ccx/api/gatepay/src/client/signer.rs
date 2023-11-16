use std::future::Future;
use std::io;
use std::pin::Pin;

use ccx_api_lib::GatepayApiCred;
use smart_string::DisplayExt;
use smart_string::SmartString;
use thiserror::Error;

use crate::client::nonce::Nonce;
use crate::util::dt_gatepay::DtGatepay;

pub type ApiSignResult<'a> = Pin<Box<dyn Future<Output = Result<String, SignError>> + Send + 'a>>;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Sign Server error: {0}")]
    ServerError(String),
}

pub trait GatepaySigner: Sync + Send {
    fn sign_api<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        timestamp: DtGatepay,
        nonce: &'b Nonce,
        body: &'b str,
    ) -> ApiSignResult<'a>;

    fn sign_payment<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        timestamp: DtGatepay,
        nonce: &'b Nonce,
        body: &'b str,
    ) -> ApiSignResult<'a>;

    fn api_key(&self) -> &str;

    fn client_id(&self) -> &str;
}

impl GatepaySigner for GatepayApiCred {
    fn sign_api<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        timestamp: DtGatepay,
        nonce: &'b Nonce,
        body: &'b str,
    ) -> ApiSignResult<'a> {
        Box::pin(async move { Ok(sign(&self.auth_key, timestamp, nonce, body)) })
    }

    fn sign_payment<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        timestamp: DtGatepay,
        nonce: &'b Nonce,
        body: &'b str,
    ) -> ApiSignResult<'a> {
        Box::pin(async move { Ok(sign(&self.payment_key, timestamp, nonce, body)) })
    }

    fn api_key(&self) -> &str {
        &self.api_key
    }

    fn client_id(&self) -> &str {
        &self.client_id
    }
}

pub fn sign(secret: &str, timestamp: DtGatepay, nonce: &Nonce, body: &str) -> String {
    use hex::ToHex;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Sha512;

    let mut mac =
        Hmac::<Sha512>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    let payload: SmartString<254> =
        format_args!("{}\n{}\n{}\n", timestamp.timestamp(), nonce.as_ref(), body).to_fmt();

    mac.update(payload.as_bytes());

    let digest = mac.finalize().into_bytes();
    digest.encode_hex()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [source](https://www.gate.io/docs/gatepay/common/en/#_2-4-2-signature-algorithm)
    const _REFERENCE_PYTHON_IMPLEMENTATION: &str = r#"

import hashlib
import hmac
def generate_signature(timestamp, nonce, body, secret):
    """
    :GenerateSignature generates the request signature
    :timestamp: UTC timestamp converted to a string, precision is millisecond
    :nonce: random string
    :body: request body
    :secretKey: api_secret provided by GateIO
    :return: string signature
    """
    payload = '%s\n%s\n%s\n' % (timestamp, nonce, body)
    signed = hmac.new(secret.encode(), payload.encode(), hashlib.sha512)
    return signed.digest().hex()

generate_signature('1700073707111', '456', '{the_answer: 42}', '123')    
    "#;

    #[test]
    fn test_sign() {
        let timestamp = DtGatepay::from_timestamp(1700073707111);
        let nonce = Nonce::new("456".to_string());
        let hex_digest = sign("123", timestamp, &nonce, "{the_answer: 42}");
        assert_eq!(
            hex_digest,
            "1b03f547af065490c6e106bd93afc861329c52b596512d49489ca0c4ea57adee\
             2991a576e402422a6bb092b83acf0a5618f5b0a7d4c639ae7779fbb3a1b16346"
        );
    }
}
