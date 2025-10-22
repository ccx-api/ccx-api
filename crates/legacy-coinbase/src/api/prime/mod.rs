use std::time::Duration;

use url::Url;

// use crate::client::WebsocketStream;
use crate::client::CCX_COINBASE_PRIME_API_PREFIX;
use crate::client::PrimeApiCred;
use crate::client::PrimeConfig;
use crate::client::PrimeRateLimiter;
use crate::client::Proxy;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBucketMode;
use crate::client::RestPrimeClient;

pub const API_BASE: &str = "https://api.prime.coinbase.com/";
pub const STREAM_BASE: &str = "wss://api.prime.coinbase.com/";

pub const API_BASE_TESTNET: &str = "https://api-public.sandbox.prime.coinbase.com/";
pub const STREAM_BASE_TESTNET: &str = "wss://api-public.sandbox.prime.coinbase.com/";

pub const RL_PORTFOLIO_KEY: &str = "portfolio";
pub const RL_PORTFOLIO_INTERVAL: Duration = Duration::from_secs(1);
pub const RL_PORTFOLIO_LIMIT: u32 = 25;
// pub const RL_PORTFOLIO_BURST: u32 = 50;

// TODO mod error;
mod activity;
mod address_book;
mod asset;
mod order;
mod portfolio;
mod product;
mod transaction;
pub mod types;
mod wallet;

pub use activity::*;
pub use address_book::*;
pub use asset::*;
pub use order::*;
pub use portfolio::*;
pub use product::*;
pub use transaction::*;
pub use wallet::*;

mod prelude {
    #[cfg(feature = "with_network")]
    pub use super::PrimeApi;
    pub use super::types::*;
    pub use crate::DtCoinbasePrime;
    pub use crate::api::prelude::*;
    pub use crate::api::prime::RL_PORTFOLIO_KEY;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::env_var_with_prefix;

    use super::*;
    use crate::client::CoinbasePrimeSigner;
    use crate::client::PrimeRateLimiterBuilder;

    #[derive(Clone)]
    pub struct PrimeApi<S: CoinbasePrimeSigner = PrimeApiCred> {
        pub(crate) client: RestPrimeClient<S>,
        pub(crate) rate_limiter: PrimeRateLimiter,
    }

    impl PrimeApi<PrimeApiCred> {
        /// Reads config from env vars with names like:
        /// "CCX_COINBASE_API_KEY", "CCX_COINBASE_API_SECRET", "CCX_COINBASE_API_PASSPHRASE",
        /// "CCX_COINBASE_API_TESTNET"
        pub fn from_env() -> PrimeApi<PrimeApiCred> {
            let testnet = PrimeConfig::<PrimeApiCred>::env_var("TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_PRIME_API_PREFIX);
            PrimeApi::new(
                PrimeApiCred::from_env_with_prefix(CCX_COINBASE_PRIME_API_PREFIX),
                testnet,
                proxy,
            )
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", "${prefix}_PASSPHRASE", "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> PrimeApi<PrimeApiCred> {
            let testnet = env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            PrimeApi::new(PrimeApiCred::from_env_with_prefix(prefix), testnet, proxy)
        }
    }

    impl<S> PrimeApi<S>
    where
        S: CoinbasePrimeSigner,
    {
        pub fn new(signer: S, testnet: bool, proxy: Option<Proxy>) -> Self {
            let (api_base, stream_base) = if testnet {
                (API_BASE_TESTNET, STREAM_BASE_TESTNET)
            } else {
                (API_BASE, STREAM_BASE)
            };
            let api_base = Url::parse(api_base).unwrap();
            let stream_base = Url::parse(stream_base).unwrap();
            PrimeApi::with_config(PrimeConfig::new(signer, api_base, stream_base, proxy))
        }

        pub fn with_config(config: PrimeConfig<S>) -> Self {
            // let limits = config.tier.limits();
            let client = RestPrimeClient::new(config);

            let rate_limiter = PrimeRateLimiterBuilder::default()
                .bucket(
                    RL_PORTFOLIO_KEY,
                    RateLimiterBucket::default()
                        .mode(RateLimiterBucketMode::Interval)
                        .delay(RL_PORTFOLIO_INTERVAL)
                        .interval(RL_PORTFOLIO_INTERVAL)
                        .limit(RL_PORTFOLIO_LIMIT),
                )
                .start();

            PrimeApi {
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
