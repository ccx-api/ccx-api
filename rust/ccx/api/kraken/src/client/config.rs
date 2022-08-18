use url::Url;

use crate::client::KrakenSigner;
use ccx_api_lib::env_var_with_prefix;
pub use ccx_api_lib::ApiCred;
pub use ccx_api_lib::Proxy;

pub static CCX_KRAKEN_API_PREFIX: &str = "CCX_KRAKEN_API";

pub(crate) struct RateLimiterTierLimits {
    pub(crate) private: u32,
    pub(crate) matching_engine: u32,
}

#[derive(Clone)]
pub enum RateLimiterTier {
    Starter,
    Intermediate,
    Pro,
}

/// Values are calculated by adding the Maximum Counter value and the expected count
/// decay (in a minute) of a given tier.
///
/// Reference:
/// - API Rate Limits: https://support.kraken.com/hc/en-us/articles/206548367-What-are-the-API-rate-limits
/// - Matching Engine Limits: https://support.kraken.com/hc/en-us/articles/360045239571
///
/// STARTER:        PRIVATE = 15 + 20 (0.33 * 60), MATCHING_ENGINE = 60 + 60
/// INTERMEDIATE:   PRIVATE = 20 + 30 (0.5 * 60), MATCHING_ENGINE = 125 + 140
/// PRO:            PRIVATE = 20 + 60 (1 * 60), MATCHING_ENGINE = 180 + 225
impl RateLimiterTier {
    pub(crate) fn limits(&self) -> RateLimiterTierLimits {
        match self {
            Self::Starter => RateLimiterTierLimits {
                private: 15 + 20,
                matching_engine: 60 + 60,
            },
            Self::Intermediate => RateLimiterTierLimits {
                private: 20 + 30,
                matching_engine: 125 + 140,
            },
            Self::Pro => RateLimiterTierLimits {
                private: 20 + 60,
                matching_engine: 180 + 225,
            },
        }
    }
}

/// API config.
#[derive(Clone)]
pub struct Config<S: KrakenSigner> {
    pub signer: S,
    pub api_base: Url,
    pub stream_base: Url,
    pub proxy: Option<Proxy>,
    pub tier: RateLimiterTier,
}

impl<S> Config<S>
where
    S: KrakenSigner,
{
    pub fn new(
        signer: S,
        api_base: Url,
        stream_base: Url,
        proxy: Option<Proxy>,
        tier: RateLimiterTier,
    ) -> Self {
        Config {
            signer: signer.into(),
            api_base,
            stream_base,
            proxy,
            tier,
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
