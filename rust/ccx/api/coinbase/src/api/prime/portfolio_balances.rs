use chrono::Utc;
use uuid::Uuid;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};

use super::prelude::*;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioBalancesResponse {
    /// A list of balances.
    pub balances: Vec<AccountPortfolioBalances>,
    pub r#type: PortfolioBalanceType,
    pub trading_balances: AccountPortfolioTradingBalances,
    pub vault_balances: AccountPortfolioVaultBalances,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum PortfolioBalanceType {
    /// Trading balances.
    #[serde(rename = "TRADING_BALANCES")]
    TradingBalances,
    /// Vault balances.
    #[serde(rename = "VAULT_BALANCES")]
    VaultBalances,
    /// Total balances (The sum of vault and trading).
    #[serde(rename = "TOTAL_BALANCES")]
    TotalBalances,
}
#[cfg(feature = "db")]
forward_display_to_serde!(AccountPortfolioBalanceType);
#[cfg(feature = "db")]
forward_from_str_to_serde!(AccountPortfolioBalanceType);

// impl PortfolioBalanceType {
//     pub fn from_name(name: &str) -> Option<Self> {
//         Self::from_str(name).ok()
//     }
//
//     pub fn name(&self) -> String {
//         self.to_string()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioBalances {
    /// The display symbol for the asset.
    pub symbol: Atom,
    /// The total amount in whole units with full precision.
    pub amount: Decimal,
    /// Amount that is currently held in obligation to an open order's position
    /// or a pending withdrawal.
    pub holds: Decimal,
    pub bonded_amount: Decimal,
    pub reserved_amount: Decimal,
    pub unbonding_amount: Decimal,
    pub unvested_amount: Decimal,
    pub pending_rewards_amount: Decimal,
    pub past_rewards_amount: Decimal,
    pub bondable_amount: Decimal,
    pub withdrawable_amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioTradingBalances {
    /// The total amount in whole units with full precision.
    pub total: Decimal,
    /// Amount that is currently held in obligation to an open order's position
    /// or a pending withdrawal.
    pub holds: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioVaultBalances {
    /// The total amount in whole units with full precision.
    pub total: Decimal,
    /// Amount that is currently held in obligation to an open order's position
    /// or a pending withdrawal.
    pub holds: Decimal,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Get Account Balance.
    ///
    /// Retrieve all cash balances, net of pending withdrawals.
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `symbols` - A list of symbols by which to filter the response.
    /// * `balance_type` - A type by which to filter balances.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getposttradecredit]
    pub fn get_portfolio_balances(
        &self,
        portfolio_id: Uuid,
        symbols: Option<String>,
        balance_type: Option<PortfolioBalanceType>,
    ) -> CoinbaseResult<Task<AccountPortfolioBalancesResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/balances");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("symbols", &symbols)?
                    .try_query_arg("balance_type", &balance_type)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
