use serde::{Deserialize, Serialize};
use url::Url;

use ccx_api_lib::env_var_with_prefix;
pub use ccx_api_lib::ApiCred;
pub use ccx_api_lib::Proxy;

pub static CCX_BINANCE_API_PREFIX: &str = "CCX_BINANCE_API";

/// API config.
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub cred: ApiCred,
    pub api_base: Url,
    pub stream_base: Url,
    pub proxy: Option<Proxy>,
}

impl Config {
    pub fn new(cred: ApiCred, api_base: Url, stream_base: Url, proxy: Option<Proxy>) -> Self {
        Config {
            cred,
            api_base,
            stream_base,
            proxy,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_BINANCE_API_PREFIX, postfix)
    }
}
