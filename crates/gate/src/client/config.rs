pub use ccx_api_lib::PrimeApiCred;
pub use ccx_api_lib::Proxy;
use ccx_api_lib::env_var_with_prefix;
use url::Url;

pub static CCX_GATE_API_PREFIX: &str = "CCX_GATE_API";

/// API config.
#[derive(Clone)]
pub struct GateApiConfig<S> {
    pub signer: S,
    pub api_base: Url,
    pub stream_base: Url,
    pub proxy: Option<Proxy>,
    // pub tier: RateLimiterTier,
}

impl<S> GateApiConfig<S> {
    pub fn new(
        signer: S,
        api_base: Url,
        stream_base: Url,
        proxy: Option<Proxy>,
        // tier: RateLimiterTier,
    ) -> Self {
        GateApiConfig {
            signer,
            api_base,
            stream_base,
            proxy,
            // tier,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_GATE_API_PREFIX, postfix)
    }
}
