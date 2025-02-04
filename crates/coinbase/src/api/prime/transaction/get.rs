use super::Transaction;
use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TransactionResponse {
    pub transaction: Transaction,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Get Transaction by Transaction ID.
    ///
    /// Retrieve a specific transaction by its transaction ID
    /// (only transactions that affect balances are accessible).
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `transaction_id` - The transaction ID.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_gettransaction]
    pub fn get_transaction(
        &self,
        portfolio_id: Uuid,
        transaction_id: Uuid,
    ) -> CoinbaseResult<Task<TransactionResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/transactions/{transaction_id}");
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
