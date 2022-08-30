use url::Url;

use ccx_api_lib::env_var_with_prefix;

use crate::client::ApiCred;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBuilder;
use crate::client::RestClient;
use crate::client::WebsocketStream;
use crate::client::CCX_BINANCE_API_PREFIX;
use crate::error::*;

mod account;
mod broker;
mod margin;
mod market_data;
mod user_data_stream;
// TODO mod error;
// TODO mod savings;
// TODO mod mining;
mod futures;
// TODO mod blvt;
// TODO mod bswap;
mod clearjunction;
mod subaccount;
pub mod util;
mod wallet;
mod websocket_market;

pub use self::account::*;
pub use self::broker::*;
pub use self::futures::*;
pub use self::margin::*;
pub use self::market_data::*;
pub use self::user_data_stream::*;
// TODO pub use self::error::*;
// TODO pub use self::savings::*;
// TODO pub use self::mining::*;
// TODO pub use self::blvt::*;
// TODO pub use self::bswap::*;
#[cfg(feature = "experimental")]
pub use self::clearjunction::*;
pub use self::subaccount::*;
pub use self::wallet::*;
pub use self::websocket_market::*;
use crate::client::BinanceSigner;

pub const API_BASE: &str = "https://api.binance.com/";
pub const STREAM_BASE: &str = "wss://stream.binance.com/stream";

pub const API_BASE_TESTNET: &str = "https://testnet.binance.vision/";
pub const STREAM_BASE_TESTNET: &str = "wss://testnet.binance.vision/stream";

pub const RL_WEIGHT_PER_MINUTE: &str = "weight_per_minute";
pub const RL_ORDERS_PER_SECOND: &str = "orders_per_second";
pub const RL_ORDERS_PER_DAY: &str = "orders_per_day";

pub enum RlPriorityLevel {
    Normal = 1,
    High = 2,
}

mod prelude {
    pub use crate::api::prelude::*;

    #[cfg(feature = "with_network")]
    pub use super::SpotApi;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use crate::client::RateLimiter;

    use super::*;

    #[derive(Clone)]
    pub struct SpotApi<S>
    where
        S: BinanceSigner,
    {
        pub client: RestClient<S>,
        pub(crate) rate_limiter: RateLimiter,
    }

    impl<S> SpotApi<S>
    where
        S: BinanceSigner,
    {
        pub fn new(signer: S, testnet: bool, proxy: Option<Proxy>) -> Self {
            let (api_base, stream_base) = if testnet {
                (
                    Url::parse(API_BASE_TESTNET).unwrap(),
                    Url::parse(STREAM_BASE_TESTNET).unwrap(),
                )
            } else {
                (
                    Url::parse(API_BASE).unwrap(),
                    Url::parse(STREAM_BASE).unwrap(),
                )
            };
            SpotApi::with_config(Config::new(signer, api_base, stream_base, proxy))
        }

        /// Reads config from env vars with names like:
        /// "CCX_BINANCE_API_KEY", "CCX_BINANCE_API_SECRET", and "CCX_BINANCE_API_TESTNET"
        pub fn from_env() -> SpotApi<ApiCred> {
            let testnet = Config::<S>::env_var("TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
            SpotApi::new(
                ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
                testnet,
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", and "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> SpotApi<ApiCred> {
            let testnet = env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            SpotApi::new(ApiCred::from_env_with_prefix(prefix), testnet, proxy)
        }

        pub fn with_config(config: Config<S>) -> Self {
            use std::time::Duration;

            let client = RestClient::new(config);
            let rate_limiter = RateLimiterBuilder::default()
                .bucket(
                    RL_WEIGHT_PER_MINUTE,
                    RateLimiterBucket::default()
                        .interval(Duration::from_secs(60))
                        .limit(1_200),
                )
                .bucket(
                    RL_ORDERS_PER_SECOND,
                    RateLimiterBucket::default()
                        .delay(Duration::from_secs(1))
                        .interval(Duration::from_secs(1))
                        .limit(10),
                )
                .bucket(
                    RL_ORDERS_PER_DAY,
                    RateLimiterBucket::default()
                        .interval(Duration::from_secs(86_400))
                        .limit(200_000),
                )
                .start();

            SpotApi {
                client,
                rate_limiter,
            }
        }

        /// Creates multiplexed websocket stream.
        pub async fn ws(&self) -> BinanceResult<WebsocketStream> {
            self.client.web_socket().await
        }
    }
}
