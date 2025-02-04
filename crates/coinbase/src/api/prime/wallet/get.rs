use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioWallet;

/// List all wallets associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWalletResponse {
    /// A list of balances.
    pub wallet: AccountPortfolioWallet,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Get Wallet by Wallet ID.
    ///
    /// Retrieve a specific wallet by Wallet ID.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `wallet_id` - The wallet ID.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getwallet]
    pub fn get_wallet(
        &self,
        portfolio_id: Uuid,
        wallet_id: Uuid,
    ) -> CoinbaseResult<Task<AccountPortfolioWalletResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/wallets/{wallet_id}");
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
