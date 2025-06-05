use macro_rules_attribute::apply;
use uuid::Uuid;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::derive::Response;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
/// Request to cancel an existing order
pub struct CancelOrder {
    /// The id of the trading account
    account_id: String,
    /// The id of the order to cancel
    order_id: Uuid,
}

#[apply(Response)]
pub struct CancelOrderResponse {}

impl Response for CancelOrderResponse {}

impl Request for CancelOrder {
    type Response = CancelOrderResponse;

    const HTTP_METHOD: http::Method = http::Method::PUT;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;
        let order_id = self.order_id;

        format!("/api/prime/trading/v1/accounts/{account_id}/orders/{order_id}/cancel").into()
    }
}

impl SignedRequest for CancelOrder {}
