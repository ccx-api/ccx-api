//! [https://docs.cloud.bitstamp.com//docs/rate-limits]

use std::time::Duration;

use url::Url;

use crate::BitstampResult;
use crate::client::ApiCred;
use crate::client::CCX_BITSTAMP_API_PREFIX;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RateLimiter;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBucketMode;
use crate::client::RestClient;
use crate::client::WebsocketStream;

pub const API_BASE: &str = "https://www.bitstamp.net/api/v2/";
pub const STREAM_BASE: &str = "wss://ws.bitstamp.net";

pub const API_BASE_TESTNET: &str = "https://www.bitstamp.net/api/v2/";
pub const STREAM_BASE_TESTNET: &str = "wss://ws.bitstamp.net";

pub const RL_GENERAL_KEY: &str = "GENERAL";
pub const RL_GENERAL_INTERVAL: Duration = Duration::from_secs(60);
pub const RL_GENERAL_LIMIT: u32 = 800;

pub mod account_balance;
pub mod currency;
pub mod fee;
pub mod order;
pub mod order_book;
pub mod trading_pair;

mod prelude {
    pub use rust_decimal::Decimal;
    pub use serde::Serialize;
    pub use uuid::Uuid;

    // pub use super::types::*;
    #[cfg(feature = "with_network")]
    pub use super::Api;
    pub use crate::client::Task;
    pub use crate::error::*;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::env_var_with_prefix;

    use super::*;
    use crate::client::BitstampSigner;
    use crate::client::RateLimiterBuilder;

    #[derive(Clone)]
    pub struct Api<S: BitstampSigner = ApiCred> {
        pub(crate) client: RestClient<S>,
        pub(crate) rate_limiter: RateLimiter,
    }

    impl Api<ApiCred> {
        /// Reads config from env vars with names like:
        /// "CCX_BITSTAMP_API_KEY", "CCX_BITSTAMP_API_SECRET", "CCX_BITSTAMP_API_PASSPHRASE",
        /// "CCX_BITSTAMP_API_TESTNET"
        pub fn from_env() -> Api<ApiCred> {
            let testnet = Config::<ApiCred>::env_var("TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(CCX_BITSTAMP_API_PREFIX);
            Api::new(
                ApiCred::from_env_with_prefix(CCX_BITSTAMP_API_PREFIX),
                testnet,
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", "${prefix}_PASSPHRASE", "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> Api<ApiCred> {
            let testnet = env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            Api::new(ApiCred::from_env_with_prefix(prefix), testnet, proxy)
        }
    }

    impl<S> Api<S>
    where
        S: BitstampSigner,
    {
        pub fn new(signer: S, testnet: bool, proxy: Option<Proxy>) -> Self {
            let (api_base, stream_base) = if testnet {
                (API_BASE_TESTNET, STREAM_BASE_TESTNET)
            } else {
                (API_BASE, STREAM_BASE)
            };
            let api_base = Url::parse(api_base).unwrap();
            let stream_base = Url::parse(stream_base).unwrap();
            Api::with_config(Config::new(signer, api_base, stream_base, proxy))
        }

        pub fn with_config(config: Config<S>) -> Self {
            // let limits = config.tier.limits();
            let client = RestClient::new(config);

            // Do not make more than 8000 requests per 10 minutes or we will ban your IP address.
            // For real time data please refer to the websocket API.
            // [https://www.bitstamp.net/api/#request-limits]
            let rate_limiter = RateLimiterBuilder::default()
                .bucket(
                    RL_GENERAL_KEY,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::Interval)
                        .delay(Duration::ZERO)
                        .interval(RL_GENERAL_INTERVAL)
                        .limit(RL_GENERAL_LIMIT),
                )
                .start();

            Api {
                client,
                rate_limiter,
            }
        }

        /// Creates multiplexed websocket stream.
        pub async fn ws(&self) -> BitstampResult<WebsocketStream> {
            self.client.web_socket().await
        }
    }
}
