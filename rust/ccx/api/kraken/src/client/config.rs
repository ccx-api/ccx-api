use serde::{Deserialize, Serialize};
use url::Url;

use ccx_api_lib::env_var_with_prefix;
pub use ccx_api_lib::ApiCred;
pub use ccx_api_lib::Proxy;
use ccx_api_lib::Signer;

pub static CCX_KRAKEN_API_PREFIX: &str = "CCX_KRAKEN_API";

/// API config.
#[derive(Clone)]
pub struct Config {
    pub signer: Signer,
    pub api_base: Url,
    pub stream_base: Url,
    pub proxy: Option<Proxy>,
}

impl Config {
    pub fn new(
        signer: impl Into<Signer>,
        api_base: Url,
        stream_base: Url,
        proxy: Option<Proxy>,
    ) -> Self {
        Config {
            signer: signer.into(),
            api_base,
            stream_base,
            proxy,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_KRAKEN_API_PREFIX, postfix)
    }

    pub(crate) fn api_key(&self) -> &str {
        match self.signer {
            Signer::Cred(ref cred) => cred.key.as_str(),
            Signer::Hook(ref closure) => closure.api_key.as_str(),
        }
    }

    pub(crate) fn signer(&self) -> &Signer {
        &self.signer
    }
}
