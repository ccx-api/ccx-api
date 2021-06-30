use url::Url;

use crate::client::{ApiCred, Config, RestClient, WebsocketStream};
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
    pub use crate::api::prelude::*;

    #[cfg(feature = "with_network")]
    pub use super::UmApi;
}

pub const API_BASE: &str = "https://fapi.binance.com/";
pub const STREAM_BASE: &str = "wss://fstream.binance.com/stream";

pub const API_BASE_TESTNET: &str = "https://testnet.binancefuture.com/";
pub const STREAM_BASE_TESTNET: &str = "wss://stream.binancefuture.com/stream";

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    #[derive(Clone)]
    pub struct UmApi {
        pub client: RestClient,
    }

    impl UmApi {
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
            UmApi::with_config(Config::new(cred, api_base, stream_base))
        }

        /// Reads config from env vars with names like:
        /// "CCX_BINANCE_API_KEY", "CCX_BINANCE_API_SECRET", and "CCX_BINANCE_API_TESTNET"
        pub fn from_env() -> Self {
            let testnet = Config::env_var("TESTNET").as_deref() == Some("1");
            UmApi::new(ApiCred::from_env(), testnet)
        }

        /// Reads config from env vars with names like:
        /// "${prefix}_KEY", "${prefix}_SECRET", and "${prefix}_TESTNET"
        pub fn from_env_with_prefix(prefix: &str) -> Self {
            let testnet = Config::env_var_with_prefix(prefix, "TESTNET").as_deref() == Some("1");
            UmApi::new(ApiCred::from_env_with_prefix(prefix), testnet)
        }

        pub fn with_config(config: Config) -> Self {
            let client = RestClient::new(config);
            UmApi { client }
        }

        /// Creates multiplexed websocket stream.
        pub async fn ws(&self) -> LibResult<WebsocketStream> {
            self.client.web_socket2().await
        }
    }
}
