use super::Transaction;
use super::TransactionType;
use crate::api::prime::prelude::*;

/// List all wallets associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfoliWalletTransactionsResponse {
    /// A list of transactions.
    pub transactions: Vec<Transaction>,
    pub pagination: NextPage,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # List Wallet Transactions.
    ///
    /// Retrieve transactions for a given wallet
    /// (only transactions that affect balances are accessible).
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `wallet_id` - The wallet ID.
    /// * `types` - The transaction types by which to filter the response.
    /// * `start_time` - UTC timestamp from which to filter the response (inclusive, ISO-8601 format).
    /// * `end_time` - UTC timestamp until which to filter the response (exclusive, ISO-8601 format).
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getwallettransactions]
    pub fn list_transactions(
        &self,
        portfolio_id: Uuid,
        wallet_id: Uuid,
        types: &[TransactionType],
        start_time: Option<DtCoinbasePrime>,
        end_time: Option<DtCoinbasePrime>,
        page: Page,
    ) -> CoinbaseResult<Task<PortfoliWalletTransactionsResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/transactions");
        let types: Option<String> = if types.is_empty() {
            None
        } else {
            Some(
                types
                    .iter()
                    .map(|a| a.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        };
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("types", &types)?
                    .try_query_arg("start_time", &start_time)?
                    .try_query_arg("end_time", &end_time)?
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
