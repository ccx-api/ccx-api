use url::Url;

use crate::client::CoinbaseTradeSigner;
use ccx_api_lib::env_var_with_prefix;
pub use ccx_api_lib::ApiCred;
pub use ccx_api_lib::Proxy;

pub static CCX_COINBASE_TRADE_API_PREFIX: &str = "CCX_COINBASE_TRADE_API";

// pub(crate) struct RateLimiterTierLimit {
//     pub(crate) private: RateLimiterTierLimitValue,
//     pub(crate) matching_engine: RateLimiterTierLimitValue,
// }

// pub(crate) struct RateLimiterTierLimitValue {
//     pub(crate) max: u32,
//     pub(crate) period: Duration,
// }

// #[derive(Clone)]
// pub enum RateLimiterTier {
//     Starter,
//     Intermediate,
//     Pro,
// }

// /// Reference:
// /// - API Rate Limits: https://support.kraken.com/hc/en-us/articles/206548367-What-are-the-API-rate-limits
// /// - Matching Engine Limits: https://support.kraken.com/hc/en-us/articles/360045239571
// ///
// /// STARTER:        PRIVATE = 15 (-0.33/1sec), MATCHING_ENGINE = 60/1min
// /// INTERMEDIATE:   PRIVATE = 20 (-0.33/1sec, MATCHING_ENGINE = 140/1min
// /// PRO:            PRIVATE = 20 (-1/1sec), MATCHING_ENGINE = 225/1min
// impl RateLimiterTier {
//     pub(crate) fn limits(&self) -> RateLimiterTierLimit {
//         match self {
//             Self::Starter => RateLimiterTierLimit {
//                 private: RateLimiterTierLimitValue {
//                     max: 15,
//                     period: Duration::from_secs(3),
//                 },
//                 matching_engine: RateLimiterTierLimitValue {
//                     max: 60,
//                     period: Duration::from_secs(60),
//                 },
//             },
//             Self::Intermediate => RateLimiterTierLimit {
//                 private: RateLimiterTierLimitValue {
//                     max: 20,
//                     period: Duration::from_secs(2),
//                 },
//                 matching_engine: RateLimiterTierLimitValue {
//                     max: 140,
//                     period: Duration::from_secs(60),
//                 },
//             },
//             Self::Pro => RateLimiterTierLimit {
//                 private: RateLimiterTierLimitValue {
//                     max: 20,
//                     period: Duration::from_secs(1),
//                 },
//                 matching_engine: RateLimiterTierLimitValue {
//                     max: 225,
//                     period: Duration::from_secs(60),
//                 },
//             },
//         }
//     }
// }

/// API config.
#[derive(Clone)]
pub struct TradeConfig<S: CoinbaseTradeSigner> {
    pub signer: S,
    pub api_base: Url,
    pub stream_base: Url,
    pub proxy: Option<Proxy>,
    // pub tier: RateLimiterTier,
}

impl<S> TradeConfig<S>
    where
        S: CoinbaseTradeSigner,
{
    pub fn new(
        signer: S,
        api_base: Url,
        stream_base: Url,
        proxy: Option<Proxy>,
        // tier: RateLimiterTier,
    ) -> Self {
        TradeConfig {
            signer: signer.into(),
            api_base,
            stream_base,
            proxy,
            // tier,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_COINBASE_TRADE_API_PREFIX, postfix)
    }

    pub(crate) fn api_key(&self) -> &str {
        &self.signer.api_key()
    }

    pub(crate) fn signer(&self) -> &S {
        &self.signer
    }
}
