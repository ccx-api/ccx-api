use macro_rules_attribute::apply;
use uuid::Uuid;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::order::Order;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct GetOrder {
    /// The id of the trading account to retrieve
    #[serde(skip)]
    account_id: String,
    /// The id of the order to retrieve
    #[serde(skip)]
    order_id: Uuid,
}

impl Response for Order {}

impl Request for GetOrder {
    type Response = Order;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;
        let order_id = self.order_id;

        format!("/api/prime/trading/v1/accounts/{account_id}/orders/{order_id}").into()
    }
}

impl SignedRequest for GetOrder {}
