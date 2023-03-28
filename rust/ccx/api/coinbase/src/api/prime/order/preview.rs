use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioOrder;
use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderTimeInForce;
use crate::api::prime::PortfolioOrderType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
struct AccountPortfolioOrderPreviewRequest<'a> {
    portfolio_id: Uuid,
    product_id: &'a str,
    side: PortfolioOrderSide,
    r#type: PortfolioOrderType,
    base_quantity: Option<Decimal>,
    quote_value: Option<Decimal>,
    limit_price: Option<Decimal>,
    start_time: Option<&'a str>,
    expiry_time: Option<&'a str>,
    time_in_force: Option<PortfolioOrderTimeInForce>,
    is_raise_exact: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioOrderPreviewResponse {
    pub order: AccountPortfolioOrder,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Get Order Preview.
    ///
    /// Retrieve an order preview.
    ///
    /// * `portfolio_id` - The ID of the portfolio under which the order was placed.
    /// * `product_id` -
    /// * `side` -
    /// * `type` -
    /// * `base_quantity` -
    /// * `quote_value` -
    /// * `limit_price` -
    /// * `start_time` -
    /// * `expiry_time` -
    /// * `time_in_force` -
    /// * `is_raise_exact` -
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_orderpreview]
    pub fn get_order_preview(
        &self,
        portfolio_id: Uuid,
        product_id: &str,
        side: PortfolioOrderSide,
        r#type: PortfolioOrderType,
        base_quantity: Option<Decimal>,
        quote_value: Option<Decimal>,
        limit_price: Option<Decimal>,
        start_time: Option<&str>,
        expiry_time: Option<&str>,
        time_in_force: Option<PortfolioOrderTimeInForce>,
        is_raise_exact: Option<bool>,
    ) -> CoinbaseResult<Task<AccountPortfolioOrderPreviewResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/order_preview");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(AccountPortfolioOrderPreviewRequest {
                        portfolio_id,
                        product_id,
                        side,
                        r#type,
                        base_quantity,
                        quote_value,
                        limit_price,
                        start_time,
                        expiry_time,
                        time_in_force,
                        is_raise_exact,
                    })?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
