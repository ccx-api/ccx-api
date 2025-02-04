use base64::engine::general_purpose;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;

use crate::env_var_with_prefix;

/// Exchange API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ExchangeApiCred {
    pub key: String,
    pub secret: Vec<u8>,
    pub passphrase: String,
}

impl ExchangeApiCred {
    pub fn new(key: Option<String>, secret: Option<String>, passphrase: Option<String>) -> Self {
        ExchangeApiCred {
            key: key.unwrap_or_default(),
            // TODO new() -> Result<_>.
            secret: Self::decode_secret(secret.as_deref()),
            passphrase: passphrase.unwrap_or_default(),
        }
    }

    pub fn decode_secret(secret: Option<&str>) -> Vec<u8> {
        secret
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
            .unwrap_or_default()
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        ExchangeApiCred::new(
            env_var_with_prefix(prefix, "KEY"),
            env_var_with_prefix(prefix, "SECRET"),
            env_var_with_prefix(prefix, "PASSPHRASE"),
        )
    }
}

/// Prime API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PrimeApiCred {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

impl PrimeApiCred {
    pub fn new(key: Option<String>, secret: Option<String>, passphrase: Option<String>) -> Self {
        PrimeApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
            passphrase: passphrase.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        PrimeApiCred::new(
            env_var_with_prefix(prefix, "KEY"),
            env_var_with_prefix(prefix, "SECRET"),
            env_var_with_prefix(prefix, "PASSPHRASE"),
        )
    }
}

/// Advanced Trade API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiCred {
    pub key: String,
    pub secret: String,
}

impl ApiCred {
    pub fn new(key: Option<String>, secret: Option<String>) -> Self {
        ApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        ApiCred::new(
            env_var_with_prefix(prefix, "KEY"),
            env_var_with_prefix(prefix, "SECRET"),
        )
    }
}
