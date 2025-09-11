use std::time::Duration;

use ccx_api_lib::env_var_with_prefix;
use url::Url;

use crate::client::ApiCred;
use crate::client::BinanceSigner;
use crate::client::CCX_BINANCE_API_PREFIX;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RateLimiter;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBuilder;
use crate::client::RestClient;
use crate::client::WebsocketStream;
use crate::error::*;

mod market_data;
// TODO mod websocket_market;
// TODO mod account;
// TODO mod user_data_stream;
// TODO mod error;

pub use self::market_data::*;
// pub use self::websocket_market::*;
// pub use self::account::*;
// pub use self::user_data_stream::*;

mod prelude {
    #[cfg(feature = "with_network")]
    pub use super::UmApi;
    pub use crate::api::prelude::*;
}

pub const API_BASE: &str = "https://fapi.binance.com/";
pub const STREAM_BASE: &str = "wss://fstream.binance.com/stream";

pub const API_BASE_TESTNET: &str = "https://testnet.binancefuture.com/";
pub const STREAM_BASE_TESTNET: &str = "wss://stream.binancefuture.com/stream";

pub const RL_WEIGHT_PER_MINUTE: &str = "weight_per_minute";

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    #[derive(Clone)]
    pub struct UmApi<S>
    where
        S: BinanceSigner,
    {
        pub(crate) client: RestClient<S>,
        pub(crate) rate_limiter: RateLimiter,
    }

    impl<S> UmApi<S>
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
            UmApi::with_config(Config::new(signer, api_base, stream_base, proxy))
        }

        /// Reads config from env vars with names like:
        /// "CCX_BINANCE_API_KEY", "CCX_BINANCE_API_SECRET", and "CCX_BINANCE_API_TESTNET"
        pub fn from_env() -> UmApi<ApiCred> {
            let testnet = Config::<S>::env_var("TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
            UmApi::new(
                ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
                testnet,
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", and "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> UmApi<ApiCred> {
            let testnet = env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            UmApi::new(ApiCred::from_env_with_prefix(prefix), testnet, proxy)
        }

        pub fn with_config(config: Config<S>) -> Self {
            let client = RestClient::new(config);
            let rate_limiter = RateLimiterBuilder::default()
                .bucket(
                    RL_WEIGHT_PER_MINUTE,
                    RateLimiterBucket::default()
                        .interval(Duration::from_secs(60))
                        .limit(1_200),
                )
                .start();
            UmApi {
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
