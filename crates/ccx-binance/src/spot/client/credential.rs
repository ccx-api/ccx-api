use std::sync::Arc;

use hmac::Hmac;
use hmac::Mac;
use hmac::digest::InvalidLength;
use sha2::Sha256;

use crate::spot::client::signer::BinanceSpotSigner;
use crate::spot::error::BinanceSpotError;

#[derive(Clone)]
pub struct BinanceSpotCredential {
    inner: Arc<Credential>,
}

struct Credential {
    key_name: String,
    api_key: String,
    mac: Hmac<Sha256>,
}

impl BinanceSpotCredential {
    /// Create a new BinanceSpotCredential
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
        Ok(BinanceSpotCredential { inner })
    }

    pub fn key_name(&self) -> &str {
        &self.inner.key_name
    }
}

impl BinanceSpotSigner for BinanceSpotCredential {
    fn api_key(&self) -> &str {
        self.inner.api_key.as_str()
    }

    async fn sign_request(&self, query: &str) -> Result<String, BinanceSpotError> {
        let mut mac = self.inner.mac.clone();
        mac.update(query.as_bytes());
        let res = mac.finalize().into_bytes();
        Ok(format!("{res:x}"))
    }
}
