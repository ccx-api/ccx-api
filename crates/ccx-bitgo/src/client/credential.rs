use serde::{Deserialize, Serialize};

use super::signer::BitGoSigner;

/// BitGo API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BitGoCredential {
    pub(crate) token: String,
}

impl BitGoCredential {
    pub fn new(token: String) -> Self {
        BitGoCredential { token }
    }
}

impl BitGoSigner for BitGoCredential {
    fn token(&self) -> &str {
        &self.token
    }
}
