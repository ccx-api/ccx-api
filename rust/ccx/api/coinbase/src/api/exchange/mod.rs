//! [https://docs.cloud.coinbase.com/exchange/docs/rate-limits]

use std::time::Duration;

use url::Url;

use crate::client::ExchangeApiCred;
use crate::client::ExchangeConfig;
use crate::client::ExchangeRateLimiter;
use crate::client::Proxy;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBucketMode;
use crate::client::RestExchangeClient;
use crate::client::CCX_COINBASE_EXCHANGE_API_PREFIX;

pub const API_BASE: &str = "https://api.exchange.coinbase.com/";
pub const STREAM_MARKET_BASE: &str = "wss://ws-feed.exchange.coinbase.com/";
pub const STREAM_DIRECT_BASE: &str = "wss://ws-direct.exchange.coinbase.com/";

pub const API_BASE_TESTNET: &str = "https://api-public.sandbox.exchange.coinbase.com/";
pub const STREAM_MARKET_BASE_TESTNET: &str = "wss://ws-feed-public.sandbox.exchange.coinbase.com/";
pub const STREAM_DIRECT_BASE_TESTNET: &str = "wss://ws-direct.sandbox.exchange.coinbase.com/";

pub const RL_PUBLIC_KEY: &str = "PUBLIC";
pub const RL_PUBLIC_INTERVAL: Duration = Duration::from_secs(1);
pub const RL_PUBLIC_LIMIT: u32 = 10;
pub const RL_PUBLIC_BURST_LIMIT: u32 = 15;

pub const RL_PRIVATE_KEY: &str = "PRIVATE";
pub const RL_PRIVATE_INTERVAL: Duration = Duration::from_secs(1);
pub const RL_PRIVATE_LIMIT: u32 = 15;
pub const RL_PRIVATE_BURST_LIMIT: u32 = 30;

pub const RL_PRIVATE_FILLS_KEY: &str = "PRIVATE_FILLS";
pub const RL_PRIVATE_FILLS_INTERVAL: Duration = Duration::from_secs(1);
pub const RL_PRIVATE_FILLS_LIMIT: u32 = 10;
pub const RL_PRIVATE_FILLS_BURST_LIMIT: u32 = 20;

// TODO mod error;
mod account;
mod account_coinbase;
mod address_book;
mod currency;
mod fees;
mod order;
mod product;
mod profile;
mod transfer;
pub mod types;

pub use account::*;
pub use account_coinbase::*;
pub use address_book::*;
pub use currency::*;
pub use fees::*;
pub use order::*;
pub use product::*;
pub use profile::*;
pub use transfer::*;

pub mod prelude {
    #[cfg(feature = "with_network")]
    pub use super::ExchangeApi;
    #[cfg(feature = "with_network")]
    pub(crate) use super::RL_PRIVATE_KEY;
    pub use crate::api::prelude::*;
    pub use crate::DtCoinbaseEx;
    pub use crate::DtCoinbasePrime;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::env_var_with_prefix;

    use super::*;
    use crate::client::CoinbaseExchangeSigner;
    use crate::client::ExchangeRateLimiterBuilder;

    #[derive(Clone)]
    pub struct ExchangeApi<S: CoinbaseExchangeSigner = ExchangeApiCred> {
        pub(crate) client: RestExchangeClient<S>,
        pub(crate) rate_limiter: ExchangeRateLimiter,
    }

    impl ExchangeApi<ExchangeApiCred> {
        /// Reads config from env vars with names like:
        /// "CCX_COINBASE_API_KEY", "CCX_COINBASE_API_SECRET", "CCX_COINBASE_API_PASSPHRASE",
        /// "CCX_COINBASE_API_TESTNET"
        pub fn from_env() -> ExchangeApi<ExchangeApiCred> {
            let testnet =
                ExchangeConfig::<ExchangeApiCred>::env_var("TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_EXCHANGE_API_PREFIX);
            ExchangeApi::new(
                ExchangeApiCred::from_env_with_prefix(CCX_COINBASE_EXCHANGE_API_PREFIX),
                testnet,
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", "${prefix}_PASSPHRASE", "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> ExchangeApi<ExchangeApiCred> {
            let testnet = env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            ExchangeApi::new(
                ExchangeApiCred::from_env_with_prefix(prefix),
                testnet,
                proxy,
            )
        }
    }

    impl<S> ExchangeApi<S>
    where
        S: CoinbaseExchangeSigner,
    {
        pub fn new(signer: S, testnet: bool, proxy: Option<Proxy>) -> Self {
            let (api_base, stream_market_base, stream_direct_base) = if testnet {
                (
                    API_BASE_TESTNET,
                    STREAM_MARKET_BASE_TESTNET,
                    STREAM_DIRECT_BASE_TESTNET,
                )
            } else {
                (API_BASE, STREAM_MARKET_BASE, STREAM_DIRECT_BASE)
            };
            let api_base = Url::parse(api_base).unwrap();
            let _stream_market_base = Url::parse(stream_market_base).unwrap();
            let _stream_direct_base = Url::parse(stream_direct_base).unwrap();
            ExchangeApi::with_config(ExchangeConfig::new(
                signer, api_base, /* , stream_base */
                proxy,
            ))
        }

        pub fn with_config(config: ExchangeConfig<S>) -> Self {
            // let limits = config.tier.limits();
            let client = RestExchangeClient::new(config);

            // Advanced Exchange API endpoints are throttled by IP at 10 requests per second.
            // https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-rate-limits
            let rate_limiter = ExchangeRateLimiterBuilder::default()
                .bucket(
                    RL_PUBLIC_KEY,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::Interval)
                        .delay(RL_PUBLIC_INTERVAL)
                        .interval(RL_PUBLIC_INTERVAL)
                        .limit(RL_PUBLIC_LIMIT),
                )
                .bucket(
                    RL_PRIVATE_KEY,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::Interval)
                        .delay(RL_PRIVATE_INTERVAL)
                        .interval(RL_PRIVATE_INTERVAL)
                        .limit(RL_PRIVATE_LIMIT),
                )
                .bucket(
                    RL_PRIVATE_FILLS_KEY,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::Interval)
                        .delay(RL_PRIVATE_FILLS_INTERVAL)
                        .interval(RL_PRIVATE_FILLS_INTERVAL)
                        .limit(RL_PRIVATE_FILLS_LIMIT),
                )
                .start();

            ExchangeApi {
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
