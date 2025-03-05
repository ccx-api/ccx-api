use ccx_lib::SignError;
use hex::ToHex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use smart_string::SmartString;

use super::signer::{GateSigner, GateSignerPayload};

/// Gate.io API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GateCredential {
    pub(crate) key: String,
    pub(crate) secret: String,
}

impl GateCredential {
    pub fn new(key: String, secret: String) -> Self {
        GateCredential { key, secret }
    }
}

impl GateSigner for GateCredential {
    fn api_key(&self) -> &str {
        &self.key
    }

    async fn sign_request(&self, payload: GateSignerPayload<'_>) -> Result<String, SignError> {
        Ok(sign(
            &self.secret,
            payload.method.as_str(),
            payload.path,
            payload.query.unwrap_or_default(),
            payload.body.unwrap_or_default(),
            payload.timestamp.to_string().as_str(),
        ))
    }
}

/// Generate signature string. It needs to be signed using the secret
///
/// [Api documentation](https://www.gate.io/docs/developers/apiv4/#apiv4-signed-request-requirements)
///
/// In APIv4, signature string is concatenated as the following way:
///
/// ```text
/// Request Method + "\n" + Request URL + "\n" + Query String + "\n" + HexEncode(SHA512(Request Payload)) + "\n" + Timestamp`
/// ```
fn signature_string(
    method: &str,
    path: &str,
    query: &str,
    payload: &str,
    timestamp: &str,
) -> String {
    let mut sha = sha2::Sha512::new();
    sha.update(payload.as_bytes());
    let hex_sha512_payload: SmartString<128> = sha.finalize().encode_hex();
    format!("{method}\n{path}\n{query}\n{hex_sha512_payload}\n{timestamp}")
}

fn sign(
    secret: &str,
    method: &str,
    path: &str,
    query: &str,
    payload: &str,
    timestamp: &str,
) -> String {
    use hex::ToHex;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Sha512;

    let mut mac =
        Hmac::<Sha512>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(signature_string(method, path, query, payload, timestamp).as_bytes());
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
    fn sign_task() {
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

    #[test]
    fn sign_list_all_orders() {
        let hex_digest = sign(
            "secret",
            "GET",
            "/api/v4/futures/orders",
            "contract=BTC_USD&status=finished&limit=50",
            "",
            "1541993715",
        );
        assert_eq!(
            hex_digest,
            "55f84ea195d6fe57ce62464daaa7c3c02fa9d1dde954e4c898289c9a2407a3d6\
             fb3faf24deff16790d726b66ac9f74526668b13bd01029199cc4fcc522418b8a"
        );
    }

    #[test]
    fn sign_create_new_order() {
        let hex_digest = sign(
            "secret",
            "POST",
            "/api/v4/futures/orders",
            "",
            r#"{"contract":"BTC_USD","type":"limit","size":100,"price":6800,"time_in_force":"gtc"}"#,
            "1541993715",
        );
        assert_eq!(
            hex_digest,
            "eae42da914a590ddf727473aff25fc87d50b64783941061f47a3fdb92742541f\
             c4c2c14017581b4199a1418d54471c269c03a38d788d802e2c306c37636389f0"
        );
    }
}
