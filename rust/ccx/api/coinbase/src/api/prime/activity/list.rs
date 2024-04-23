use super::Activity;
use super::ActivityCategory;
use super::ActivityStatus;
use crate::api::prime::prelude::*;

/// List all activities associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioActivitiesResponse {
    /// A list of activities.
    pub activities: Vec<Activity>,
    pub pagination: NextPage,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// List Activities.
    ///
    /// List all activities associated with a given entity.
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `symbols` - List of symbols by which to filter the response (e.g. \["ETH-USD"\]).
    /// * `categories` - List of activity categories to filter by (e.g. [order, transaction, account, admin, allocation])
    /// * `statuses` - List of activity statuses to filter by.
    /// * `start_date` - A start date for the activities to be queried from.
    /// * `end_date` - An end date for the activities to be queried from.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getportfolioactivities]
    pub fn list_activities(
        &self,
        portfolio_id: Uuid,
        symbols: &[Atom],
        categories: &[ActivityCategory],
        statuses: &[ActivityStatus],
        start_date: Option<DtCoinbasePrime>,
        end_date: Option<DtCoinbasePrime>,
        page: Page,
    ) -> CoinbaseResult<Task<PortfolioActivitiesResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/activities");
        let symbols: Option<String> = if symbols.is_empty() {
            None
        } else {
            Some(
                symbols
                    .iter()
                    .map(|a| a.as_ref())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        };
        let categories: Option<String> = if categories.is_empty() {
            None
        } else {
            Some(
                categories
                    .iter()
                    .map(|a| a.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        };
        let statuses: Option<String> = if statuses.is_empty() {
            None
        } else {
            Some(
                statuses
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
                    .try_query_arg("symbols", &symbols)?
                    .try_query_arg("categories", &categories)?
                    .try_query_arg("statuses", &statuses)?
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
