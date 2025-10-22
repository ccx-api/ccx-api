use chrono::Utc;

use super::PortfolioCommission;
use crate::CoinbaseResult;
use crate::Uuid;
use crate::api::prime::PrimeApi;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;

pub type GetCommissionResponse = PortfolioCommission;

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Get Portfolio Commission.
    ///
    /// Retrieve commission associated with a given portfolio.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getportfoliocommission]
    pub fn get_portfolio_commission(
        &self,
        portfolio_id: Uuid,
    ) -> CoinbaseResult<Task<GetCommissionResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/commission");
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
