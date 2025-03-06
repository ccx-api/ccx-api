use std::sync::Arc;

use ccx_lib::SignError;
use hmac::Hmac;
use hmac::Mac;
use hmac::digest::InvalidLength;
use sha2::Sha256;

use crate::client::signer::BinanceSigner;

#[derive(Clone)]
pub struct BinanceCredential {
    inner: Arc<Credential>,
}

struct Credential {
    key_name: String,
    api_key: String,
    mac: Hmac<Sha256>,
}

impl BinanceCredential {
    /// Create a new BinanceCredential
    ///
    /// # Arguments
    ///
    /// * `key_name` - The name of the key (optional), used for logging.
    /// * `api_key` - The API key, used for signing requests.
    /// * `secret_key` - The secret key, used for signing requests.
    pub fn new(
        key_name: String,
        api_key: String,
        secret_key: &[u8],
    ) -> Result<Self, InvalidLength> {
        let mac = Hmac::new_from_slice(secret_key)?;

        let inner = Arc::new(Credential {
            key_name,
            api_key,
            mac,
        });
        Ok(BinanceCredential { inner })
    }

    pub fn key_name(&self) -> &str {
        &self.inner.key_name
    }
}

impl BinanceSigner for BinanceCredential {
    fn api_key(&self) -> &str {
        self.inner.api_key.as_str()
    }

    async fn sign_request(&self, query: &str) -> Result<String, SignError> {
        let mut mac = self.inner.mac.clone();
        mac.update(query.as_bytes());
        let res = mac.finalize().into_bytes();
        Ok(format!("{res:x}"))
    }
}
