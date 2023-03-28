use chrono::Utc;
use uuid::Uuid;

use super::prelude::*;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioResponse {
    /// A list of portfolios.
    pub portfolio: AccountPortfolio,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolio {
    /// The unique ID of the portfolio.
    pub id: Uuid,
    /// The name of the portfolio.
    pub name: String,
    /// The ID of the entity to which the portfolio is associated.
    pub entity_id: Uuid,
    /// The ID of the organization to which the portfolio is associated.
    pub organization_id: Uuid,
}

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
    pub fn get_portfolio(&self, portfolio_id: Uuid) -> CoinbaseResult<Task<AccountPortfolioResponse>> {
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
