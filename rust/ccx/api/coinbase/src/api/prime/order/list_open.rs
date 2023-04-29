use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioOrder;
use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderType;
use crate::dt_coinbase::DtCoinbase;

/// List all wallets associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioOrdersResponse {
    /// A list of balances.
    pub orders: Vec<AccountPortfolioOrder>,
    pub pagination: NextPage,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// List Open Orders.
    ///
    /// List all currently open orders.
    ///
    /// **Caution: The query parameters for this endpoint are temporarily being adjusted as we work
    /// to improve our API experience:**
    /// * ProductID only supports one productID (for example, BTC-USD) or none, in which case all
    ///   open orders for all productIDs are returned.
    /// * All other query params are currently non-functional and result in an error if used.
    ///   This includes: `order_type`, `cursor`, `limit`, `sort_direction`, `start_date`,
    ///   `order_side`, and `end_date`.
    /// * The maximum number of orders returned is 1000. If a client has more than 1000 open orders,
    ///   an error is returned prompting the user to use Websocket API, or FIX API to stream open
    ///   orders.
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `product_ids` - List of products by which to filter the response (e.g. \["ETH-USD"\]).
    /// * `order_type` - Order type by which to filter the response.
    /// * `order_side` - An order side to filter on.
    /// * `start_date` - A start date for the orders to be queried from.
    /// * `end_date` - An end date for the orders to be queried from.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getopenorders]
    #[allow(clippy::too_many_arguments)]
    pub fn list_open_orders_(
        &self,
        portfolio_id: Uuid,
        product_ids: &[Atom],
        order_type: Option<PortfolioOrderType>,
        order_side: Option<PortfolioOrderSide>,
        start_date: &DtCoinbase,
        end_date: Option<&str>,
        page: Page,
    ) -> CoinbaseResult<Task<AccountPortfolioOrdersResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/open_orders");
        let product_ids = if product_ids.is_empty() {
            None
        } else {
            Some(
                product_ids
                    .iter()
                    .map(|a| a.as_ref())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        };
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("product_ids", &product_ids)?
                    .try_query_arg("order_type", &order_type)?
                    .try_query_arg("order_side", &order_side)?
                    .query_arg("start_date", &start_date)?
                    .try_query_arg("end_date", &end_date)?
                    .try_query_arg("cursor", &page.cursor())?
                    .try_query_arg("limit", &page.limit())?
                    .try_query_arg("sort_direction", &page.sort_direction())?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }

    /// List Open Orders.
    ///
    /// List all currently open orders.
    ///
    /// **Caution: The query parameters for this endpoint are temporarily being adjusted as we work
    /// to improve our API experience:**
    /// * ProductID only supports one productID (for example, BTC-USD) or none, in which case all
    ///   open orders for all productIDs are returned.
    /// * All other query params are currently non-functional and result in an error if used.
    ///   This includes: `order_type`, `cursor`, `limit`, `sort_direction`, `start_date`,
    ///   `order_side`, and `end_date`.
    /// * The maximum number of orders returned is 1000. If a client has more than 1000 open orders,
    ///   an error is returned prompting the user to use Websocket API, or FIX API to stream open
    ///   orders.
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `product_id` - A product by which to filter the response (e.g. "ETH-USD").
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getopenorders]
    pub fn list_open_orders(
        &self,
        portfolio_id: Uuid,
        product_id: Option<Atom>,
    ) -> CoinbaseResult<Task<AccountPortfolioOrdersResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/open_orders");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("product_ids", &product_id)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
