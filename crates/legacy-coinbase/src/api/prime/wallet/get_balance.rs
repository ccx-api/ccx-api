use crate::api::prime::AccountPortfolioWalletBalance;
use crate::api::prime::prelude::*;

/// List all wallets associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWalletBalanceResponse {
    /// A list of balances.
    pub balance: AccountPortfolioWalletBalance,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Get Wallet Balance.
    ///
    /// Query balance for a specific wallet.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `wallet_id` - The wallet ID.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getwalletbalance]
    pub fn get_wallet_balance(
        &self,
        portfolio_id: Uuid,
        wallet_id: Uuid,
    ) -> CoinbaseResult<Task<AccountPortfolioWalletBalanceResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/balance");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}

#[cfg(test)]
mod tests {
    use ccx_api_lib::dec;

    use super::*;

    #[test]
    fn test_deserialize_wallet_balance_response_doc() {
        let json = r#"{
          "balance": {
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
          }
        }"#;
        let expected: AccountPortfolioWalletBalanceResponse =
            AccountPortfolioWalletBalanceResponse {
                balance: AccountPortfolioWalletBalance {
                    symbol: "BTC".into(),
                    amount: dec!(109.42),
                    holds: dec!(2),
                    bonded_amount: Some(dec!(109.42)),
                    reserved_amount: Some(dec!(109.42)),
                    unbonding_amount: Some(dec!(109.42)),
                    unvested_amount: Some(dec!(109.42)),
                    pending_rewards_amount: Some(dec!(109.42)),
                    past_rewards_amount: Some(dec!(109.42)),
                    bondable_amount: Some(dec!(109.42)),
                    withdrawable_amount: Some(dec!(109.42)),
                    fiat_amount: None,
                },
            };
        let deserialized: AccountPortfolioWalletBalanceResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
