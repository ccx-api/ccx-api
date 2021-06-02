use crate::client::{ApiCred, Config, RestClient, WebsocketStream};
use crate::error::*;

mod margin_account_trade;
mod market_data;
mod spot_account_trade;
mod user_data_stream;
// TODO mod error;
// TODO mod savings;
// TODO mod mining;
// TODO mod futures;
// TODO mod blvt;
// TODO mod bswap;
mod clearjunction;
mod subaccount;
mod wallet;

pub use self::margin_account_trade::*;
pub use self::market_data::*;
pub use self::spot_account_trade::*;
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

mod prelude {
    pub use super::Api;

    pub use crate::error::*;
    pub use crate::proto::*;
    pub use crate::Atom;
    pub use crate::TimeWindow;

    pub use rust_decimal::Decimal;
    pub use serde::Serialize;
}

#[derive(Clone, Default)]
pub struct Api {
    pub client: RestClient,
}

impl Api {
    pub fn new() -> Self {
        Api::default()
    }

    pub fn from_env() -> Self {
        Api::with_config(Config::from_env())
    }

    pub fn with_cred(cred: ApiCred) -> Self {
        Api::with_config(Config {
            cred,
            ..Config::default()
        })
    }

    pub fn with_config(config: Config) -> Self {
        let client = RestClient::with_config(config);
        Api { client }
    }

    /// Creates multiplexed websocket stream.
    pub async fn ws(&self) -> LibResult<WebsocketStream> {
        self.client.web_socket2().await
    }
}
