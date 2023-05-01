use chrono::Utc;
use uuid::Uuid;

use super::PortfolioInfo;
use crate::api::prime::PrimeApi;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;
use crate::CoinbaseResult;

pub type GetPortfolioResponse = PortfolioInfo;

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
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getportfolio]
    pub fn get_portfolio(&self, portfolio_id: Uuid) -> CoinbaseResult<Task<GetPortfolioResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        // In the doc `/v1/portfolios/{portfolio_id}/`, but the ending slash leads to 404.
        let endpoint = format!("/v1/portfolios/{portfolio_id}");
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
