use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWalletBalance {
    /// The display symbol for the asset
    pub symbol: Atom,
    /// The total amount in whole units with full precision. Includes the `holds` amount.
    pub amount: Decimal,
    /// Amount that is currently held in obligation to an open order's position or a pending withdrawal
    pub holds: Decimal,
    /// Amount that is currently locked due to bonding/staking, potentially subject to an unbonding period, in whole units
    #[serde(default, with = "maybe_str")]
    pub bonded_amount: Option<Decimal>,
    /// Amount that must remain in the wallet due to the protocol, in whole units
    #[serde(default, with = "maybe_str")]
    pub reserved_amount: Option<Decimal>,
    /// Amount that is in the process of unbonding, in whole units
    #[serde(default, with = "maybe_str")]
    pub unbonding_amount: Option<Decimal>,
    /// Unrealized amount subject to a vesting schedule, in whole units
    #[serde(default, with = "maybe_str")]
    pub unvested_amount: Option<Decimal>,
    /// Pending bonding/staking rewards that have not yet been realized, in whole units
    #[serde(default, with = "maybe_str")]
    pub pending_rewards_amount: Option<Decimal>,
    /// Previously realized bonding/staking rewards, in whole units
    #[serde(default, with = "maybe_str")]
    pub past_rewards_amount: Option<Decimal>,
    /// Amount available for bonding/staking, in whole units
    #[serde(default, with = "maybe_str")]
    pub bondable_amount: Option<Decimal>,
    /// Amount available to withdraw, in whole units
    #[serde(default, with = "maybe_str")]
    pub withdrawable_amount: Option<Decimal>,
    /// Undocumented*
    #[serde(default, with = "maybe_str")]
    pub fiat_amount: Option<Decimal>,
}

#[cfg(test)]
mod tests {
    use ccx_coinbase_examples_util::d;

    use super::*;

    #[test]
    fn test_deserialize_wallet_balance_doc() {
        let json = r#"{
            "symbol": "BTC",
            "amount": "109.42",
            "holds": "2",
            "bonded_amount": "109.42",
            "reserved_amount": "109.42",
            "unbonding_amount": "109.42",
            "unvested_amount": "109.42",
            "pending_rewards_amount": "109.42",
            "past_rewards_amount": "109.42",
            "bondable_amount": "109.42",
            "withdrawable_amount": "109.42"
        }"#;
        let expected: AccountPortfolioWalletBalance = AccountPortfolioWalletBalance {
            symbol: "BTC".into(),
            amount: d("109.42"),
            holds: d("2"),
            bonded_amount: Some(d("109.42")),
            reserved_amount: Some(d("109.42")),
            unbonding_amount: Some(d("109.42")),
            unvested_amount: Some(d("109.42")),
            pending_rewards_amount: Some(d("109.42")),
            past_rewards_amount: Some(d("109.42")),
            bondable_amount: Some(d("109.42")),
            withdrawable_amount: Some(d("109.42")),
            fiat_amount: None,
        };
        let deserialized: AccountPortfolioWalletBalance = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_deserialize_wallet_balance_live() {
        let json = r#"{
            "symbol":"USDT",
            "amount":"5309.4992533441396509",
            "holds":"0",
            "bonded_amount":"",
            "reserved_amount":"",
            "unbonding_amount":"",
            "unvested_amount":"",
            "pending_rewards_amount":"",
            "past_rewards_amount":"",
            "bondable_amount":"",
            "withdrawable_amount":"5309.4992533441396509",
            "fiat_amount":""
        }"#;
        let expected: AccountPortfolioWalletBalance = AccountPortfolioWalletBalance {
            symbol: "USDT".into(),
            amount: d("5309.4992533441396509"),
            holds: d("0"),
            bonded_amount: None,
            reserved_amount: None,
            unbonding_amount: None,
            unvested_amount: None,
            pending_rewards_amount: None,
            past_rewards_amount: None,
            bondable_amount: None,
            withdrawable_amount: Some(d("5309.4992533441396509")),
            fiat_amount: None,
        };
        let deserialized: AccountPortfolioWalletBalance = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
