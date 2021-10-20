use std::collections::HashMap;

use super::prelude::*;

pub const API_0_PRIVATE_BALANCE: &str = "/0/private/Balance";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountBalanceResponse {
    /// Account Balance
    #[serde(flatten)]
    pub asset: HashMap<Atom, Decimal>,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl SpotApi {
        /// Get Account Balance.
        ///
        /// Retrieve all cash balances, net of pending withdrawals.
        pub async fn get_account_balance(&self, nonce: Nonce) -> KrakenApiResult<AccountBalanceResponse> {
            self.client.post(API_0_PRIVATE_BALANCE)?
                .signed(nonce)?
                .request_body(())?
                .send().await
        }
    }
}
