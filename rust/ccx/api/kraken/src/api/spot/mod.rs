use std::time::Duration;
use url::Url;

use crate::client::ApiCred;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RateLimiter;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBucketMode;
use crate::client::RateLimiterTier;
use crate::client::RestClient;
use crate::client::WebsocketStream;
use crate::client::CCX_KRAKEN_API_PREFIX;

// TODO mod error;
// TODO mod savings;
mod market_data;
mod user_data;
mod user_funding;
mod user_trading;
// TODO mod user_staking;
// pub mod util;
pub mod types;
// TODO mod websocket_auth;

pub use self::market_data::*;
pub use self::types::*;
pub use self::user_data::*;
pub use self::user_funding::*;
pub use self::user_trading::*;
use crate::client::KrakenSigner;

pub const API_BASE: &str = "https://api.kraken.com/";
pub const STREAM_BASE: &str = "wss://ws.kraken.com/";

pub const RL_PUBLIC_PER_SECOND: &str = "public";
pub const RL_PRIVATE_PER_MINUTE: &str = "private";
pub const RL_MATCHING_ENGINE_PER_MINUTE: &str = "matching_engine";

pub enum RlPriorityLevel {
    Normal = 1,
    High = 2,
}

mod prelude {
    pub use super::types::*;
    pub use crate::api::prelude::*;

    #[cfg(feature = "with_network")]
    pub use super::SpotApi;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use crate::{client::RateLimiterBuilder, KrakenResult};

    use super::*;

    #[derive(Clone)]
    pub struct SpotApi<S: KrakenSigner = ApiCred> {
        pub(crate) client: RestClient<S>,
        pub(crate) rate_limiter: RateLimiter,
    }

    impl SpotApi<ApiCred> {
        /// Reads config from env vars with names like:
        /// "CCX_KRAKEN_API_KEY", "CCX_KRAKEN_API_SECRET"
        pub fn from_env() -> SpotApi<ApiCred> {
            let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
            let tier = RateLimiterTier::Starter;
            SpotApi::new(
                ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX),
                proxy,
                tier,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET"
        pub fn from_env_with_prefix(prefix: &str) -> SpotApi<ApiCred> {
            let proxy = Proxy::from_env_with_prefix(prefix);
            let tier = RateLimiterTier::Starter;
            SpotApi::new(ApiCred::from_env_with_prefix(prefix), proxy, tier)
        }
    }

    impl<S> SpotApi<S>
    where
        S: KrakenSigner,
    {
        pub fn new(signer: S, proxy: Option<Proxy>, tier: RateLimiterTier) -> Self {
            let api_base = Url::parse(API_BASE).unwrap();
            let stream_base = Url::parse(STREAM_BASE).unwrap();
            SpotApi::with_config(Config::new(signer, api_base, stream_base, proxy, tier))
        }

        pub fn with_config(config: Config<S>) -> Self {
            let limits = config.tier.limits();
            let client = RestClient::new(config);

            let rate_limiter = RateLimiterBuilder::default()
                .bucket(
                    RL_PUBLIC_PER_SECOND,
                    RateLimiterBucket::default()
                        .delay(Duration::from_secs(10))
                        .interval(Duration::from_secs(10))
                        .limit(10),
                )
                .bucket(
                    RL_PRIVATE_PER_MINUTE,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::KrakenDecrease)
                        .interval(limits.private.period)
                        .limit(limits.private.max),
                )
                .bucket(
                    RL_MATCHING_ENGINE_PER_MINUTE,
                    RateLimiterBucket::default()
                        .interval(limits.matching_engine.period)
                        .limit(limits.matching_engine.max),
                )
                .start();

            SpotApi {
                client,
                rate_limiter,
            }
        }

        /// Creates multiplexed websocket stream.
        pub async fn ws(&self) -> KrakenResult<WebsocketStream> {
            self.client.web_socket().await
        }
    }
}
