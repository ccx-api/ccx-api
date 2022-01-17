use url::Url;

use ccx_api_lib::env_var_with_prefix;

use crate::client::SignBinance;
pub use ccx_api_lib::ApiCred;
pub use ccx_api_lib::Proxy;

pub static CCX_BINANCE_API_PREFIX: &str = "CCX_BINANCE_API";

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
        env_var_with_prefix(CCX_BINANCE_API_PREFIX, postfix)
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

#[derive(Clone)]
pub struct Hook {
    pub(crate) api_key: String,
    pub(crate) closure: Box<dyn SignBinance>,
}

impl Hook {
    pub fn new(api_key: String, closure: Box<dyn SignBinance>) -> Self {
        Self { api_key, closure }
    }
}

#[derive(Clone)]
pub enum Signer {
    Cred(ApiCred),
    Hook(Hook),
}

impl From<ApiCred> for Signer {
    fn from(cred: ApiCred) -> Self {
        Signer::Cred(cred)
    }
}

impl From<Hook> for Signer {
    fn from(hook: Hook) -> Self {
        Signer::Hook(hook)
    }
}
