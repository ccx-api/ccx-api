use ccx_api_lib::env_var_with_prefix;
use url::Url;

use crate::client::ApiCred;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBuilder;
use crate::client::RestClient;
use crate::client::WebsocketStream;
use crate::client::CCX_MEXC_API_PREFIX;
use crate::error::*;

mod account;
mod market_data;
mod user_data_stream;
mod wallet;

pub use self::account::*;
pub use self::market_data::*;
pub use self::user_data_stream::*;
pub use self::wallet::*;
use crate::client::MexcSigner;

pub const API_BASE: &str = "https://api.mexc.com/";
pub const STREAM_BASE: &str = "wss://wbs.mexc.com/ws";

pub const RL_WEIGHT_PER_MINUTE: &str = "weight_per_minute";
pub const RL_ORDERS_PER_SECOND: &str = "orders_per_second";
pub const RL_ORDERS_PER_DAY: &str = "orders_per_day";

pub enum RlPriorityLevel {
    Normal = 1,
    High = 2,
}

mod prelude {
    #[cfg(feature = "with_network")]
    pub use super::SpotApi;
    pub use crate::api::prelude::*;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::RateLimiter;

    #[derive(Clone)]
    pub struct SpotApi<S>
    where
        S: MexcSigner,
    {
        pub client: RestClient<S>,
        pub(crate) rate_limiter: RateLimiter,
    }

    impl<S> SpotApi<S>
    where
        S: MexcSigner,
    {
        pub fn new(signer: S, proxy: Option<Proxy>) -> Self {
            let (api_base, stream_base) = (
                    Url::parse(API_BASE).unwrap(),
                    Url::parse(STREAM_BASE).unwrap(),
                );
            SpotApi::with_config(Config::new(signer, api_base, stream_base, proxy))
        }

        /// Reads config from env vars with names like:
        /// "CCX_MEXC_API_KEY", "CCX_MEXC_API_SECRET"
        pub fn from_env() -> SpotApi<ApiCred> {
            let proxy = Proxy::from_env_with_prefix(CCX_MEXC_API_PREFIX);
            SpotApi::new(
                ApiCred::from_env_with_prefix(CCX_MEXC_API_PREFIX),
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET"
        pub fn from_env_with_prefix(prefix: &str) -> SpotApi<ApiCred> {
            let proxy = Proxy::from_env_with_prefix(prefix);
            SpotApi::new(ApiCred::from_env_with_prefix(prefix), proxy)
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
        pub async fn ws(&self) -> MexcResult<WebsocketStream> {
            self.client.web_socket().await
        }
    }
}
