use crate::api::prime::prelude::*;
use crate::api::prime::Activity;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioActivityResponse {
    pub activity: Activity,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Get Activity by Activity ID
    ///
    /// Retrieve an activity by its activity ID.
    ///
    /// * `portfolio_id` - Portfolio to retrieve activity for.
    /// * `activity_id` - Id of the activity to retrieve.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getportfolioactivity]
    pub fn get_activity(
        &self,
        portfolio_id: Uuid,
        activity_id: Uuid,
    ) -> CoinbaseResult<Task<PortfolioActivityResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/activities/{activity_id}");
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
