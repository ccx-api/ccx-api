use chrono::Utc;

use super::types::PortfolioList;
use crate::CoinbaseResult;
use crate::api::prime::PrimeApi;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;

pub type GetPortfoliosResponse = PortfolioList;

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # List Portfolios.
    ///
    /// List all portfolios for which the current API key has read access.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getportfolios]
    pub fn get_portfolios(&self) -> CoinbaseResult<Task<GetPortfoliosResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = "/v1/portfolios";
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
