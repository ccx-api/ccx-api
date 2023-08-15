#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;
use serde::Deserialize;
use serde::Serialize;

use crate::Atom;
use crate::Decimal;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioBalance {
    /// A list of balances.
    pub balances: Vec<CurrencyBalance>,
    pub r#type: BalanceType,
    pub trading_balances: TradingBalances,
    pub vault_balances: VaultBalances,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum BalanceType {
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
forward_display_to_serde!(BalanceType);
#[cfg(feature = "db")]
forward_from_str_to_serde!(BalanceType);

// impl BalanceType {
//     pub fn from_name(name: &str) -> Option<Self> {
//         Self::from_str(name).ok()
//     }
//
//     pub fn name(&self) -> String {
//         self.to_string()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CurrencyBalance {
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
pub struct TradingBalances {
    /// The total amount in whole units with full precision.
    pub total: Decimal,
    /// Amount that is currently held in obligation to an open order's position
    /// or a pending withdrawal.
    pub holds: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VaultBalances {
    /// The total amount in whole units with full precision.
    pub total: Decimal,
    /// Amount that is currently held in obligation to an open order's position
    /// or a pending withdrawal.
    pub holds: Decimal,
}
