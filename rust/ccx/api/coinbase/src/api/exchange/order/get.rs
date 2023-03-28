use crate::api::exchange::prelude::*;
use crate::api::exchange::EitherOrderId;
use crate::api::exchange::Order;
use crate::api::exchange::RL_PRIVATE_KEY;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetOrderResponse {
    pub order: Order,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get single order.
    ///
    /// Get a single order by id.
    ///
    /// API Key Permissions
    ///
    /// This endpoint requires either the "view" or "trade" permission.
    ///
    /// Orders can be queried using either the exchange assigned id or the client assigned
    /// client_oid. When using client_oid it must be preceded by the client: namespace.
    ///
    /// If the order is canceled, and if the order had no matches, the response might return the
    /// status code 404.
    ///
    /// * `order_id` - is either the exchange assigned id or the client assigned client_oid.
    ///   When using client_oid it must be preceded by the client: namespace.
    /// * `market_type` - market type which the order was traded in (e.g. "spot").
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getorder]
    pub fn get_order(
        &self,
        order_id: EitherOrderId,
        market_type: Option<&str>,
    ) -> CoinbaseResult<Task<GetOrderResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/orders/{order_id}");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("market_type", &market_type)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
