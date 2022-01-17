use url::Url;

use ccx_api_lib::Signer;

use crate::client::ApiCred;
use crate::client::Config;
use crate::client::Proxy;
use crate::client::RestClient;
// use crate::client::WebsocketStream;
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

pub const API_BASE: &str = "https://api.kraken.com/";
pub const STREAM_BASE: &str = "https://ws.binance.vision/";

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
    use super::*;

    #[derive(Clone)]
    pub struct SpotApi {
        pub client: RestClient,
    }

    impl SpotApi {
        pub fn new(signer: impl Into<Signer>, proxy: Option<Proxy>) -> Self {
            let api_base = Url::parse(API_BASE).unwrap();
            let stream_base = Url::parse(STREAM_BASE).unwrap();
            SpotApi::with_config(Config::new(signer, api_base, stream_base, proxy))
        }

        /// Reads config from env vars with names like:
        /// "CCX_KRAKEN_API_KEY", "CCX_KRAKEN_API_SECRET"
        pub fn from_env() -> Self {
            let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
            SpotApi::new(ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX), proxy)
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET"
        pub fn from_env_with_prefix(prefix: &str) -> Self {
            let proxy = Proxy::from_env_with_prefix(prefix);
            SpotApi::new(ApiCred::from_env_with_prefix(prefix), proxy)
        }

        pub fn with_config(config: Config) -> Self {
            let client = RestClient::new(config);
            SpotApi { client }
        }

        // /// Creates multiplexed websocket stream.
        // pub async fn ws(&self) -> BinanceResult<WebsocketStream> {
        //     self.client.web_socket2().await
        // }
    }
}
