#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;
use serde::Deserialize;
use serde::Serialize;

use crate::api::prime::prelude::*;

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
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub bonded_amount: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub reserved_amount: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub unbonding_amount: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub unvested_amount: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub pending_rewards_amount: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub past_rewards_amount: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub bondable_amount: Option<Decimal>,
    pub withdrawable_amount: Decimal,
    pub fiat_amount: Decimal,
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

#[cfg(test)]
mod tests {
    use ccx_api_lib::dec;

    use super::*;

    #[test]
    fn decode_portfolio_balance() {
        let json = r#"{
            "balances": [
                {
                    "symbol": "usd",
                    "amount": "0.0032324125",
                    "holds": "0",
                    "bonded_amount": "",
                    "reserved_amount": "",
                    "unbonding_amount": "",
                    "unvested_amount": "",
                    "pending_rewards_amount": "",
                    "past_rewards_amount": "",
                    "bondable_amount": "",
                    "withdrawable_amount": "0.0032324125",
                    "fiat_amount": "0"
                },
                {
                    "symbol": "usdt",
                    "amount": "5309.4992533441396509",
                    "holds": "0",
                    "bonded_amount": "",
                    "reserved_amount": "",
                    "unbonding_amount": "",
                    "unvested_amount": "",
                    "pending_rewards_amount": "",
                    "past_rewards_amount": "",
                    "bondable_amount": "",
                    "withdrawable_amount": "5309.4992533441396509",
                    "fiat_amount": "5309.21"
                },
                {
                    "symbol": "usdc",
                    "amount": "0.18021",
                    "holds": "0",
                    "bonded_amount": "",
                    "reserved_amount": "",
                    "unbonding_amount": "",
                    "unvested_amount": "",
                    "pending_rewards_amount": "",
                    "past_rewards_amount": "",
                    "bondable_amount": "",
                    "withdrawable_amount": "0.18021",
                    "fiat_amount": "0.18"
                }
            ],
            "type": "TRADING_BALANCES",
            "trading_balances": {
                "total": "5309.39",
                "holds": "0"
            },
            "vault_balances": {
                "total": "0.00",
                "holds": "0"
            }
        }"#;
        let sample = PortfolioBalance {
            balances: vec![
                CurrencyBalance {
                    symbol: Atom::from("usd"),
                    amount: dec!(0.0032324125),
                    holds: dec!(0),
                    bonded_amount: None,
                    reserved_amount: None,
                    unbonding_amount: None,
                    unvested_amount: None,
                    pending_rewards_amount: None,
                    past_rewards_amount: None,
                    bondable_amount: None,
                    withdrawable_amount: dec!(0.0032324125),
                    fiat_amount: dec!(0),
                },
                CurrencyBalance {
                    symbol: Atom::from("usdt"),
                    amount: dec!(5309.4992533441396509),
                    holds: dec!(0),
                    bonded_amount: None,
                    reserved_amount: None,
                    unbonding_amount: None,
                    unvested_amount: None,
                    pending_rewards_amount: None,
                    past_rewards_amount: None,
                    bondable_amount: None,
                    withdrawable_amount: dec!(5309.4992533441396509),
                    fiat_amount: dec!(5309.21),
                },
                CurrencyBalance {
                    symbol: Atom::from("usdc"),
                    amount: dec!(0.18021),
                    holds: dec!(0),
                    bonded_amount: None,
                    reserved_amount: None,
                    unbonding_amount: None,
                    unvested_amount: None,
                    pending_rewards_amount: None,
                    past_rewards_amount: None,
                    bondable_amount: None,
                    withdrawable_amount: dec!(0.18021),
                    fiat_amount: dec!(0.18),
                },
            ],
            r#type: BalanceType::TradingBalances,
            trading_balances: TradingBalances {
                total: dec!(5309.39),
                holds: dec!(0),
            },
            vault_balances: VaultBalances {
                total: dec!(0.00),
                holds: dec!(0),
            },
        };

        let response: PortfolioBalance = serde_json::from_str(json).unwrap();
        assert_eq!(response, sample);
    }
}
