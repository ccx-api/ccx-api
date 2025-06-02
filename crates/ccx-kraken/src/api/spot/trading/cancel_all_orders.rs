use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

/// Cancel all open orders
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct CancelAllOrders {}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CancelAllOrdersResponse {
    /// Number of orders canceled
    pub count: i32,
    /// If set, order(s) is/are pending cancellation
    pub pending: Option<bool>,
}

impl Response for CancelAllOrdersResponse {}

impl Request for CancelAllOrders {
    type Response = CancelAllOrdersResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/CancelAll";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for CancelAllOrders {}
