use std::time::Duration;

use url::Url;

use crate::client::ApiCred;
use crate::client::Proxy;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBucketMode;
use crate::client::RestTradeClient;
use crate::client::TradeConfig;
use crate::client::TradeRateLimiter;
// use crate::client::WebsocketStream;
use crate::client::CCX_COINBASE_TRADE_API_PREFIX;

pub const API_BASE: &str = "https://coinbase.com/";
pub const STREAM_BASE: &str = "wss://coinbase.com/";

pub const RL_IP_KEY: &str = "portfolio";
pub const RL_IP_INTERVAL: Duration = Duration::from_secs(1);
pub const RL_IP_LIMIT: u32 = 10;

// TODO mod error;
// mod portfolio;
// mod portfolio_balances;
// mod portfolio_credit;
// mod portfolios;
pub mod types;
// mod wallet;
// mod order;

// pub use self::portfolio_balances::*;
// pub use self::wallet::*;
// pub use self::order::*;

mod prelude {
    pub use chrono::Utc;
    pub use uuid::Uuid;

    pub use super::types::*;
    #[cfg(feature = "with_network")]
    pub use super::TradeApi;
    pub use crate::api::prelude::*;
    pub use crate::api::trade::RL_IP_KEY;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::CoinbaseTradeSigner;
    use crate::client::TradeRateLimiterBuilder;

    #[derive(Clone)]
    pub struct TradeApi<S: CoinbaseTradeSigner = ApiCred> {
        #[allow(dead_code)]
        pub(crate) client: RestTradeClient<S>,
        #[allow(dead_code)]
        pub(crate) rate_limiter: TradeRateLimiter,
    }

    impl TradeApi<ApiCred> {
        /// Reads config from env vars with names like:
        /// "CCX_COINBASE_API_KEY", "CCX_COINBASE_API_SECRET", "CCX_COINBASE_API_PASSPHRASE"
        pub fn from_env() -> TradeApi<ApiCred> {
            let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_TRADE_API_PREFIX);
            TradeApi::new(
                ApiCred::from_env_with_prefix(CCX_COINBASE_TRADE_API_PREFIX),
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", "${prefix}_PASSPHRASE"
        pub fn from_env_with_prefix(prefix: &str) -> TradeApi<ApiCred> {
            let proxy = Proxy::from_env_with_prefix(prefix);
            TradeApi::new(ApiCred::from_env_with_prefix(prefix), proxy)
        }
    }

    impl<S> TradeApi<S>
    where
        S: CoinbaseTradeSigner,
    {
        pub fn new(signer: S, proxy: Option<Proxy>) -> Self {
            let api_base = Url::parse(API_BASE).unwrap();
            let stream_base = Url::parse(STREAM_BASE).unwrap();
            TradeApi::with_config(TradeConfig::new(signer, api_base, stream_base, proxy))
        }

        pub fn with_config(config: TradeConfig<S>) -> Self {
            // let limits = config.tier.limits();
            let client = RestTradeClient::new(config);

            // Advanced Trade API endpoints are throttled by IP at 10 requests per second.
            // https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-rate-limits
            let rate_limiter = TradeRateLimiterBuilder::default()
                .bucket(
                    RL_IP_KEY,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::Interval)
                        .delay(RL_IP_INTERVAL)
                        .interval(RL_IP_INTERVAL)
                        .limit(RL_IP_LIMIT),
                )
                .start();

            TradeApi {
                client,
                rate_limiter,
            }
        }
        //
        //     /// Creates multiplexed websocket stream.
        //     pub async fn ws(&self) -> CoinbaseResult<WebsocketStream> {
        //         self.client.web_socket().await
        //     }
    }
}
