use url::Url;

use crate::client::KrakenSigner;
use ccx_api_lib::env_var_with_prefix;
pub use ccx_api_lib::ApiCred;
pub use ccx_api_lib::Proxy;

pub static CCX_KRAKEN_API_PREFIX: &str = "CCX_KRAKEN_API";

/// API config.
#[derive(Clone)]
pub struct Config<S: KrakenSigner> {
    pub signer: S,
    pub api_base: Url,
    pub stream_base: Url,
    pub proxy: Option<Proxy>,
}

impl<S> Config<S>
where
    S: KrakenSigner,
{
    pub fn new(signer: S, api_base: Url, stream_base: Url, proxy: Option<Proxy>) -> Self {
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
        &self.signer.api_key()
    }

    pub(crate) fn signer(&self) -> &S {
        &self.signer
    }
}
