use chrono::Utc;

use super::PortfolioInfo;
use crate::CoinbaseResult;
use crate::Uuid;
use crate::api::prime::PrimeApi;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;

pub type GetPortfolioResponse = PortfolioInfo;

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Get Portfolio by Portfolio ID.
    ///
    /// Retrieve a given portfolio by its portfolio ID.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getportfolio]
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
