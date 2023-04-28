use serde::Deserialize;
use serde::Serialize;

use crate::api::exchange::prelude::*;
use crate::api::exchange::Order;
use crate::api::exchange::OrderStatus;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type ListOrderResponse = Vec<Order>;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderListSortedBy {
    CreatedAt,
    Price,
    Size,
    OrderId,
    Side,
    Type,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderListSorting {
    Asc,
    Desc,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get all orders.
    ///
    /// List your current open orders. Only open or un-settled orders are returned by default.
    /// As soon as an order is no longer open and settled, it will no longer appear in the default
    /// request. Open orders may change state between the request and the response depending on
    /// market conditions.
    ///
    /// API Key Permissions
    ///
    /// This endpoint requires either the "view" or "trade" permission.
    ///
    /// * `profile_id` - Filter results by a specific profile_id.
    /// * `product_id` - Filter results by a specific product_id.
    /// * `sorted_by` - Sort criteria for results.
    /// * `sorting` - Ascending or descending order, by `sorted_by`.
    /// * `start_date` - Filter results by minimum posted date.
    /// * `end_date` - Filter results by maximum posted date.
    /// * `before` - Used for pagination. Sets start cursor to before date.
    /// * `after` - Used for pagination. Sets end cursor to after date.
    /// * `limit` - Limit on number of results to return.
    /// * `status` - Array with order statuses to filter by.
    /// * `market_type` - Market type which the order was traded in.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getorders]
    pub fn list_orders(
        &self,
        profile_id: Option<&str>,
        product_id: Option<&str>,
        sorted_by: Option<OrderListSortedBy>,
        sorting: Option<OrderListSorting>,
        // TODO start_date
        // TODO end_date
        // TODO before
        // TODO after
        limit: u32,
        status: &[OrderStatus],
        market_type: Option<&str>,
    ) -> CoinbaseResult<Task<ListOrderResponse>> {
        let endpoint = "/orders";
        Ok(self
            .rate_limiter
            .task({
                let mut builder = self
                    .client
                    .get(&endpoint)?
                    .try_query_arg("profile_id", &profile_id)?
                    .try_query_arg("product_id", &product_id)?
                    .try_query_arg("sortedBy", &sorted_by)?
                    .try_query_arg("sorting", &sorting)?
                    .query_arg("limit", &limit)?
                    .try_query_arg("market_type", &market_type)?;
                for s in status {
                    builder = builder.query_arg("status", s)?;
                }
                builder.signed_now()?.request_body(())?
            })
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
