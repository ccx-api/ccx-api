use crate::api::order::OrderStatus;
use crate::api::prelude::*;
use crate::api::RL_GENERAL_KEY;

pub type OrderStatusResponse = OrderStatus;

#[derive(Debug, Serialize)]
struct OrderStatusRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    omit_transactions: Option<bool>,
}

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Order status
    ///
    /// This call will be executed on the account (Sub or Main),
    /// to which the used API key is bound to.
    ///
    /// Order can be fetched by using either id or client_order_id parameter.
    ///
    /// For closed orders, this call only returns information for the last 30 days.
    /// 'Order not found' error will be returned for orders outside this time range.
    ///
    /// [https://www.bitstamp.net/api/#open-orders]
    pub fn order_status<C: AsRef<str>>(
        &self,
        id: Option<u64>,
        client_order_id: Option<C>,
        omit_transactions: Option<bool>,
    ) -> BitstampResult<Task<OrderStatusResponse>> {
        if id.is_none() && client_order_id.is_none() {
            Err(BitstampError::other(
                "id or client_order_id must be specified",
            ))?
        }

        let endpoint = "order_status/";
        let client_order_id = client_order_id.as_ref().map(|c| c.as_ref());

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(endpoint)?
                    .request_body(OrderStatusRequest {
                        id,
                        client_order_id,
                        omit_transactions,
                    })?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
