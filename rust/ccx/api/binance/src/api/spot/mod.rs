use url::Url;

use crate::client::{ApiCred, Config, RestClient, WebsocketStream};
use crate::error::*;

mod account;
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
pub use self::futures::*;
pub use self::margin::*;
pub use self::market_data::*;
pub use self::user_data_stream::*;
// TODO pub use self::error::*;
// TODO pub use self::savings::*;
// TODO pub use self::mining::*;
// TODO pub use self::futures::*;
// TODO pub use self::blvt::*;
// TODO pub use self::bswap::*;
#[cfg(feature = "experimental")]
pub use self::clearjunction::*;
pub use self::subaccount::*;
pub use self::wallet::*;
pub use self::websocket_market::*;

pub const API_BASE: &str = "https://api.binance.com/";
pub const STREAM_BASE: &str = "wss://stream.binance.com/stream";

pub const API_BASE_TESTNET: &str = "https://testnet.binance.vision/";
pub const STREAM_BASE_TESTNET: &str = "wss://testnet.binance.vision/stream";

mod prelude {
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
        pub fn new(cred: ApiCred, testnet: bool) -> Self {
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
            SpotApi::with_config(Config::new(cred, api_base, stream_base))
        }

        /// Reads config from env vars with names like:
        /// "CCX_BINANCE_API_KEY", "CCX_BINANCE_API_SECRET", and "CCX_BINANCE_API_TESTNET"
        pub fn from_env() -> Self {
            let testnet = Config::env_var("TESTNET").as_deref() == Some("1");
            SpotApi::new(ApiCred::from_env(), testnet)
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", and "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> Self {
            let testnet = Config::env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            SpotApi::new(ApiCred::from_env_with_prefix(prefix), testnet)
        }

        pub fn with_config(config: Config) -> Self {
            let client = RestClient::new(config);
            SpotApi { client }
        }

        /// Creates multiplexed websocket stream.
        pub async fn ws(&self) -> LibResult<WebsocketStream> {
            self.client.web_socket2().await
        }
    }
}
