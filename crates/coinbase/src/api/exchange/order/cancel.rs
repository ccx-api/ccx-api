use crate::api::exchange::prelude::*;
use crate::api::exchange::EitherOrderId;
use crate::api::exchange::Order;
use crate::api::exchange::RL_PRIVATE_KEY;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Cancel an order.
    ///
    /// Cancel a single open order by {id}.
    ///
    /// > Cancel a previously placed order
    /// >
    /// > The order must belong to the profile that the API key belongs to. If the order
    /// > had no matches during its lifetime, its record may be purged. This means the order
    /// > details is not available with GET /orders/<id>.
    ///
    /// > Caution
    /// >
    /// > To prevent a race condition when canceling an order, it is highly recommended that you
    /// > specify the product id as a query string.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "trade" permission.
    ///
    /// ## Notes
    ///
    /// Orders can be canceled using either the exchange assigned id or the client assigned
    /// client_oid. When using client_oid it must be preceded by the client: namespace.
    ///
    /// ### Response
    ///
    /// A successfully cancelled order response includes:
    ///
    /// - the order ID if the order is cancelled with the exchange assigned id,
    /// - the client assigned client_oid if the order is cancelled with client order ID.
    ///
    /// ### Cancel Reject
    ///
    /// If the order could not be canceled (already filled or previously canceled, etc.),
    /// then an error response indicates the reason in the message field.
    ///
    /// ## Parameters
    ///
    /// * `order_id` - is either the exchange assigned id or the client assigned client_oid.
    ///   When using client_oid it must be preceded by the client: namespace.
    /// * `profile_id` - Cancels orders on a specific profile
    /// * `product_id` - Optional product id of order
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_deleteorder]
    pub fn cancel_order(
        &self,
        order_id: EitherOrderId,
        profile_id: Option<&str>,
        product_id: Option<&str>,
    ) -> CoinbaseResult<Task<Uuid>> {
        let endpoint = format!("/orders/{order_id}");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .delete(&endpoint)?
                    .try_query_arg("profile_id", &profile_id)?
                    .try_query_arg("product_id", &product_id)?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
