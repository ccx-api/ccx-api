use chrono::{DateTime, Utc};
use macro_rules_attribute::apply;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::derive::Response;
use crate::types::order::Order;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct ListOrders {
    /// The id of the trading account to retrieve
    #[serde(skip)]
    account_id: String,
    offset: Option<i32>,
    limit: Option<u32>,
    /// The client-supplied order id (optional)
    client_order_id: Option<String>,
    /// Return client orders with a `creationDate` that is greater than or equal to the given timestamp
    date_gte: Option<DateTime<Utc>>,
    /// Return client orders with a `creationDate` that is less than the given timestamp
    date_lt: Option<DateTime<Utc>>,
    /// Return client orders with an order `status` that is equal to the given string
    status: Option<String>,
}

#[apply(Response)]
pub struct ListOrdersResponse {
    /// Array of orders
    #[serde(rename = "data")]
    pub orders: Vec<Order>,
}

impl Response for ListOrdersResponse {}

impl Request for ListOrders {
    type Response = ListOrdersResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;

        format!("/api/prime/trading/v1/accounts/{account_id}/orders").into()
    }
}

impl SignedRequest for ListOrders {}
