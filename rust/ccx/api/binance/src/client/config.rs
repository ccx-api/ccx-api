use std::env::var;

use url::Url;

use super::API_BASE;
use super::API_BASE_TESTNET;
use super::STREAM_BASE;
use super::STREAM_BASE_TESTNET;

pub static CCX_BINANCE_API_KEY: &str = "CCX_BINANCE_API_KEY";
pub static CCX_BINANCE_API_SECRET: &str = "CCX_BINANCE_API_SECRET";
pub static CCX_BINANCE_API_TESTNET: &str = "CCX_BINANCE_API_TESTNET";

/// API config.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
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
    pub fn new(cred: ApiCred, testnet: bool) -> Self {
        let (api_base, stream_base) = if testnet {
            (
                Url::parse(API_BASE_TESTNET).unwrap(),
                Url::parse(STREAM_BASE_TESTNET).unwrap(),
            )
        } else {
            (
                Url::parse(API_BASE).unwrap(),
                Url::parse(STREAM_BASE).unwrap(),
            )
        };
        Config {
            cred,
            api_base,
            stream_base,
        }
    }

    pub fn from_env() -> Self {
        let cred = ApiCred::from_env();
        let testnet = var(CCX_BINANCE_API_TESTNET).unwrap_or_default() == "1";
        Self::new(cred, testnet)
    }

    /// Reads config from env vars with names like:
    /// "${prefix}_KEY", "${prefix}_SECRET", and "${prefix}_TESTNET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        let cred = ApiCred::from_env_with_prefix(prefix);
        let testnet = dbg!(var(format!("{}_TESTNET", prefix))).unwrap_or_default() == "1";
        Self::new(cred, testnet)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(ApiCred::default(), false)
    }
}

impl ApiCred {
    pub fn new(key: Option<String>, secret: Option<String>) -> Self {
        ApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    pub fn from_env() -> Self {
        ApiCred {
            key: var(CCX_BINANCE_API_KEY).unwrap_or_default(),
            secret: var(CCX_BINANCE_API_SECRET).unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        ApiCred {
            key: var(format!("{}_KEY", prefix)).unwrap_or_default(),
            secret: var(format!("{}_SECRET", prefix)).unwrap_or_default(),
        }
    }
}
