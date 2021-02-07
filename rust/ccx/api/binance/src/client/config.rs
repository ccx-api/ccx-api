use std::env::var;

use url::Url;

use super::API_BASE;
use super::STREAM_BASE;

pub static CCX_BINANCE_API_KEY: &str = "CCX_BINANCE_API_KEY";
pub static CCX_BINANCE_API_SECRET: &str = "CCX_BINANCE_API_SECRET";

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

impl Default for Config {
    fn default() -> Self {
        let cred = ApiCred::default();
        let api_base = Url::parse(API_BASE).unwrap();
        let stream_base = Url::parse(STREAM_BASE).unwrap();
        Config {
            cred,
            api_base,
            stream_base,
        }
    }
}

impl ApiCred {
    pub fn new(api: Option<String>, secret: Option<String>) -> Self {
        ApiCred {
            key: api.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    pub fn from_env() -> Self {
        ApiCred {
            key: var(CCX_BINANCE_API_KEY).unwrap_or_default(),
            secret: var(CCX_BINANCE_API_SECRET).unwrap_or_default(),
        }
    }
}
