use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioOrder;
use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderStatus;
use crate::api::prime::PortfolioOrderType;

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
    /// # List Portfolio Orders.
    ///
    /// List historical orders for a given portfolio. This endpoint returns a payload with
    /// a default limit of 100 if not specified by the user. The maximum allowed limit is 3000.
    ///
    /// **Caution:** Currently, you cannot query open orders with this endpoint: use List Open Orders
    /// if you have less than 1000 open orders, otherwise use Websocket API, or FIX API to stream
    /// open orders.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `order_statuses` - List of statuses by which to filter the response.
    /// * `product_ids` - List of products by which to filter the response (e.g. \["ETH-USD"\]).
    /// * `order_type` - Order type by which to filter the response.
    /// * `order_side` - An order side to filter on.
    /// * `start_date` - A start date for the orders to be queried from.
    /// * `end_date` - An end date for the orders to be queried from.
    ///
    /// This is not a full copy of the documentation.
    /// Please refer to the official documentation for more details.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getorders]
    #[allow(clippy::too_many_arguments)]
    pub fn list_orders(
        &self,
        portfolio_id: Uuid,
        order_statuses: &[PortfolioOrderStatus],
        product_ids: &[Atom],
        order_type: Option<PortfolioOrderType>,
        order_side: Option<PortfolioOrderSide>,
        start_date: &DtCoinbasePrime,
        end_date: Option<&DtCoinbasePrime>,
        page: Page,
    ) -> CoinbaseResult<Task<AccountPortfolioOrdersResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/orders");
        let order_statuses = if order_statuses.is_empty() {
            None
        } else {
            Some(
                order_statuses
                    .iter()
                    .map(|a| a.as_ref())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        };
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
                    .try_query_arg("order_statuses", &order_statuses)?
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
}
