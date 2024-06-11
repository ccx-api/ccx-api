use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioWallet;
use crate::api::prime::PortfolioWalletType;

/// List all wallets associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWalletsResponse {
    /// A list of balances.
    pub wallets: Vec<AccountPortfolioWallet>,
    pub pagination: NextPage,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # List Portfolio Wallets.
    ///
    /// List all wallets associated with a given portfolio.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `type` - The wallet type.
    /// * `symbols` - A list of symbols by which to filter the response.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getwallets]
    pub fn get_wallets(
        &self,
        portfolio_id: Uuid,
        wallet_type: PortfolioWalletType,
        symbols: &[&str],
        page: Page,
    ) -> CoinbaseResult<Task<AccountPortfolioWalletsResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/wallets");
        let symbols: Option<String> = if symbols.is_empty() {
            None
        } else {
            Some(
                symbols
                    .iter()
                    .map(|a| a.as_ref())
                    .collect::<Vec<&str>>()
                    .join(","),
            )
        };
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .query_arg("type", &wallet_type)?
                    .try_query_arg("symbols", &symbols)?
                    .try_query_arg("cursor", &page.cursor())?
                    .try_query_arg("limit", &page.limit())?
                    .try_query_arg("sort_direction", &page.sort_direction())?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
