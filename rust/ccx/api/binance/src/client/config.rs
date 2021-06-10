use std::env::var;

use serde::{Deserialize, Serialize};
use url::Url;

pub static CCX_BINANCE_API_PREFIX: &str = "CCX_BINANCE_API";

pub static CCX_BINANCE_API_KEY: &str = "CCX_BINANCE_API_KEY";
pub static CCX_BINANCE_API_SECRET: &str = "CCX_BINANCE_API_SECRET";
pub static CCX_BINANCE_API_TESTNET: &str = "CCX_BINANCE_API_TESTNET";

/// API config.
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub cred: ApiCred,
    pub api_base: Url,
    pub stream_base: Url,
}

/// API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiCred {
    pub(super) key: String,
    pub(super) secret: String,
}

impl Config {
    pub fn new(cred: ApiCred, api_base: Url, stream_base: Url) -> Self {
        Config {
            cred,
            api_base,
            stream_base,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        Self::env_var_with_prefix(CCX_BINANCE_API_PREFIX, postfix)
    }

    pub fn env_var_with_prefix(prefix: &str, postfix: &str) -> Option<String> {
        var(format!("{}_{}", prefix, postfix)).ok()
    }
}

impl ApiCred {
    pub fn new(key: Option<String>, secret: Option<String>) -> Self {
        ApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "CCX_BINANCE_API_KEY", "CCX_BINANCE_API_SECRET"
    pub fn from_env() -> Self {
        ApiCred::new(
            Config::env_var("KEY"),
            Config::env_var("SECRET"),
        )
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        ApiCred::new(
            Config::env_var_with_prefix(prefix, "KEY"),
            Config::env_var_with_prefix(prefix, "SECRET"),
        )
    }
}
