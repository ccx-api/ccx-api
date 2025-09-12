use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderTimeInForce;
use crate::api::prime::PortfolioOrderType;
use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
struct AccountPortfolioCreateOrderRequest<'a> {
    portfolio_id: Uuid,
    product_id: &'a str,
    side: PortfolioOrderSide,
    client_order_id: &'a str,
    r#type: PortfolioOrderType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_quantity: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    quote_value: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit_price: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start_time: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiry_time: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time_in_force: Option<PortfolioOrderTimeInForce>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stp_id: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    display_quote_size: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    display_base_size: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    is_raise_exact: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioCreateOrderResponse {
    pub order_id: Uuid,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Create Order.
    ///
    /// Create an order.
    ///
    /// Always required: portfolio_id, product_id, side, client_order_id, and type.
    /// One of either base_quantity or quote_value is always required. For LIMIT and TWAP orders,
    /// limit_price is required. For TWAP orders, start_time and expiry_time are required.
    ///
    /// * `portfolio_id` - The ID of the portfolio under which the order was placed.
    /// * `product_id` -
    /// * `side` -
    /// * `client_order_id` -
    /// * `type` -
    /// * `base_quantity` -
    /// * `quote_value` -
    /// * `limit_price` -
    /// * `start_time` -
    /// * `expiry_time` -
    /// * `time_in_force` -
    /// * `stp_id` -
    /// * `display_quote_size` -
    /// * `display_base_size` -
    /// * `is_raise_exact` -
    ///
    /// This is not a full copy of the documentation.
    /// Please refer to the official documentation for more details.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_createorder]
    #[allow(clippy::too_many_arguments)]
    pub fn create_order(
        &self,
        portfolio_id: Uuid,
        product_id: &str,
        side: PortfolioOrderSide,
        client_order_id: &str,
        r#type: PortfolioOrderType,
        base_quantity: Option<Decimal>,
        quote_value: Option<Decimal>,
        limit_price: Option<Decimal>,
        start_time: Option<&str>,
        expiry_time: Option<&str>,
        time_in_force: Option<PortfolioOrderTimeInForce>,
        stp_id: Option<&str>,
        display_quote_size: Option<Decimal>,
        display_base_size: Option<Decimal>,
        is_raise_exact: Option<bool>,
    ) -> CoinbaseResult<Task<AccountPortfolioCreateOrderResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/order");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(AccountPortfolioCreateOrderRequest {
                        portfolio_id,
                        product_id,
                        side,
                        client_order_id,
                        r#type,
                        base_quantity,
                        quote_value,
                        limit_price,
                        start_time,
                        expiry_time,
                        time_in_force,
                        stp_id,
                        display_quote_size,
                        display_base_size,
                        is_raise_exact,
                    })?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
