use crate::api::prime::prelude::*;
use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderTimeInForce;
use crate::api::prime::PortfolioOrderType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
struct AccountPortfolioCreateOrderRequest<'a> {
    portfolio_id: Uuid,
    product_id: &'a str,
    side: PortfolioOrderSide,
    client_order_id: &'a str,
    r#type: PortfolioOrderType,
    base_quantity: Option<Decimal>,
    quote_value: Option<Decimal>,
    limit_price: Option<Decimal>,
    start_time: Option<&'a str>,
    expiry_time: Option<&'a str>,
    time_in_force: Option<PortfolioOrderTimeInForce>,
    stp_id: Option<&'a str>,
    display_quote_size: Option<Decimal>,
    display_base_size: Option<Decimal>,
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
    /// Create an order. Always required: portfolio_id, product_id, side, client_order_id, and type.
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
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_createorder]
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
