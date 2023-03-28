use crate::api::exchange::prelude::*;
use crate::api::exchange::CancelAfter;
use crate::api::exchange::Order;
use crate::api::exchange::OrderSide;
use crate::api::exchange::OrderStop;
use crate::api::exchange::OrderStp;
use crate::api::exchange::OrderTimeInForce;
use crate::api::exchange::OrderType;
use crate::api::exchange::RL_PRIVATE_KEY;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CreateOrderResponse {
    pub order: Order,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateOrderRequest {
    profile_id: Option<String>,
    r#type: OrderType,
    side: OrderSide,
    product_id: Atom,
    stp: Option<OrderStp>,
    stop: Option<OrderStop>,
    stop_price: Option<Decimal>,
    price: Option<Decimal>,
    size: Option<Decimal>,
    funds: Option<Decimal>,
    time_in_force: Option<OrderTimeInForce>,
    cancel_after: Option<CancelAfter>,
    post_only: Option<bool>,
    #[serde(rename = "client_oid")]
    client_order_id: Option<Uuid>,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Create a new order.
    ///
    /// Create an order. You can place two types of orders: limit and market. Orders can only
    /// be placed if your account has sufficient funds. Once an order is placed, your account funds
    /// will be put on hold for the duration of the order. How much and which funds are put on hold
    /// depends on the order type and parameters specified.
    ///
    /// CAUTION. Each profile can place a maximum of 500 open orders on a product. Once reached, the profile
    /// cannot place any new orders until the total number of open orders is below 500.
    ///
    ///
    /// API Key Permissions
    ///
    /// This endpoint requires the "trade" permission.
    ///
    ///
    /// Limit Order Parameters
    ///
    /// * `price` - Price per base currency.
    /// * `size` - Amount of base currency to buy or sell.
    /// * `time_in_force` (optional) - GTC, GTT, IOC, or FOK (default is GTC).
    /// * `cancel_after` (optional) - "min", "hour", "day" (Requires time_in_force to be GTT).
    /// * `post_only` (optional) - Post only flag (Invalid when time_in_force is IOC or FOK).
    ///
    ///
    /// Market Order Parameters
    ///
    /// * `size` - Desired amount of base currency.
    /// * `funds` - Desired amount of quote currency to use.
    ///
    /// One of size or funds is required.
    ///
    ///
    /// Product ID
    ///
    /// The product_id must match a valid product. The products list is available via
    /// the `/products` endpoint.
    ///
    ///
    /// ....
    ///
    ///     CAUTION: RTFM!
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_postorders]
    pub fn create_order(
        &self,
        profile_id: Option<String>,
        r#type: OrderType,
        side: OrderSide,
        product_id: Atom,
        stp: Option<OrderStp>,
        stop: Option<OrderStop>,
        stop_price: Option<Decimal>,
        price: Option<Decimal>,
        size: Option<Decimal>,
        funds: Option<Decimal>,
        time_in_force: Option<OrderTimeInForce>,
        cancel_after: Option<CancelAfter>,
        post_only: Option<bool>,
        client_order_id: Option<Uuid>,
    ) -> CoinbaseResult<Task<CreateOrderResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/orders");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(CreateOrderRequest {
                        profile_id,
                        r#type,
                        side,
                        product_id,
                        stp,
                        stop,
                        stop_price,
                        price,
                        size,
                        funds,
                        time_in_force,
                        cancel_after,
                        post_only,
                        client_order_id,
                    })?,
            )
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
