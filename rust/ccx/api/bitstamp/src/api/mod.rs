//! [https://docs.cloud.bitstamp.com//docs/rate-limits]

use std::time::Duration;

use url::Url;

use crate::client::ApiCred;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RateLimiter;
use crate::client::RateLimiterBucket;
use crate::client::RateLimiterBucketMode;
use crate::client::RestClient;
// use crate::client::WebsocketStream;
use crate::client::CCX_BITSTAMP_API_PREFIX;

pub const API_BASE: &str = "https://www.bitstamp.net/api/v2/";
pub const STREAM_BASE: &str = "wss://ws.bitstamp.net";

pub const API_BASE_TESTNET: &str = "https://www.bitstamp.net/api/v2/";
pub const STREAM_BASE_TESTNET: &str = "wss://ws.bitstamp.net";

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
// mod account;
// mod address_book;
// mod currency;
// mod fees;
// mod order;
// mod product;
// mod profile;
// mod transfer;
mod account_balance;
mod order_book;

pub use account_balance::*;
pub use order_book::*;
// pub use account::*;
// pub use currency::*;
// pub use fees::*;
// pub use order::*;
// pub use product::*;
// pub use profile::*;
// pub use transfer::*;

mod prelude {
    pub use chrono::Utc;
    pub use rust_decimal::prelude::Zero;
    pub use rust_decimal::Decimal;
    pub use serde::Deserialize;
    pub use serde::Serialize;
    pub use serde_repr::Deserialize_repr;
    pub use serde_repr::Serialize_repr;
    pub use uuid::Uuid;

    // pub use super::types::*;
    #[cfg(feature = "with_network")]
    pub use super::Api;
    pub use crate::client::Nonce;
    pub use crate::client::Task;
    pub use crate::error::*;
    pub use crate::proto::*;
    pub use crate::util::maybe_str;
    pub use crate::Atom;
    pub use crate::DtBitstamp;
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
            let _stream_base = Url::parse(stream_base).unwrap();
            Api::with_config(Config::new(
                signer, api_base, /* , _stream_base */
                proxy,
            ))
        }

        pub fn with_config(config: Config<S>) -> Self {
            // let limits = config.tier.limits();
            let client = RestClient::new(config);

            // Advanced  API endpoints are throttled by IP at 10 requests per second.
            // https://docs.cloud.bitstamp.com/advanced-trade-api/docs/rest-api-rate-limits
            let rate_limiter = RateLimiterBuilder::default()
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

            Api {
                client,
                rate_limiter,
            }
        }
        //
        //     /// Creates multiplexed websocket stream.
        //     pub async fn ws(&self) -> BitstampResult<WebsocketStream> {
        //         self.client.web_socket().await
        //     }
    }
}
