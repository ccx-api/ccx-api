pub use ccx_api_lib::PrimeApiCred;
pub use ccx_api_lib::Proxy;
use ccx_api_lib::env_var_with_prefix;
use url::Url;

use crate::client::signer::GatepaySigner;

pub static CCX_GATEPAY_API_PREFIX: &str = "CCX_GATEPAY_API";

/// API config.
#[derive(Clone)]
pub struct GatepayApiConfig<S: GatepaySigner> {
    pub signer: S,
    pub api_base: Url,
    // pub stream_base: Url,
    pub proxy: Option<Proxy>,
    // pub tier: RateLimiterTier,
}

impl<S> GatepayApiConfig<S>
where
    S: GatepaySigner,
{
    pub fn new(
        signer: S,
        api_base: Url,
        // stream_base: Url,
        proxy: Option<Proxy>,
        // tier: RateLimiterTier,
    ) -> Self {
        GatepayApiConfig {
            signer,
            api_base,
            // stream_base,
            proxy,
            // tier,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_GATEPAY_API_PREFIX, postfix)
    }
}
