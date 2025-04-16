use ccx_lib::SignError;
use serde::{Deserialize, Serialize};

use super::signer::{KrakenSigner, KrakenSignerPayload, Nonce};

/// Kraken API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct KrakenCredential {
    pub(crate) key: String,
    pub(crate) secret: String,
}

impl KrakenCredential {
    pub fn new(key: String, secret: String) -> Self {
        KrakenCredential { key, secret }
    }
}

impl KrakenSigner for KrakenCredential {
    fn api_key(&self) -> &str {
        &self.key
    }

    async fn sign_request(&self, payload: KrakenSignerPayload<'_>) -> Result<String, SignError> {
        Ok(sign(
            &self.secret,
            payload.path,
            payload.body,
            payload.nonce,
        ))
    }
}

fn sign(secret: &str, path: &str, body: &str, nonce: Nonce) -> String {
    use base64::Engine as _;
    use base64::engine::general_purpose;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Digest;
    use sha2::Sha256;
    use sha2::Sha512;

    let secret = general_purpose::STANDARD
        .decode(secret)
        .expect("API_SECRET should be base64 encoded");

    // let payload = serde_urlencoded::to_string(payload).expect("serialize payload");
    let mut m256 = Sha256::new();
    m256.update(nonce.to_string());
    m256.update(body);
    let payload = m256.finalize();

    let mut m512 = Hmac::<Sha512>::new_from_slice(&secret).expect("HMAC can take key of any size");
    m512.update(path.as_bytes());
    m512.update(&payload);

    let res = m512.finalize().into_bytes();
    general_purpose::STANDARD.encode(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_example() {
        let api_sign = sign(
            "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==",
            "/0/private/AddOrder",
            "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25",
            1616492376594u64,
        );

        assert_eq!(
            api_sign,
            "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ=="
        );
    }
}
