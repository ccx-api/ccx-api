use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

/// Cancel All Orders After X (Dead Man's Switch)
///
/// Provides a "Dead Man's Switch" mechanism to protect the client from network malfunction,
/// extreme latency or unexpected matching engine downtime. The client can send a request
/// with a timeout (in seconds), that will start a countdown timer which will cancel all
/// client orders when the timer expires.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct CancelAllOrdersAfter {
    /// Timeout in seconds. Use 0 to disable the timer. Recommended: call every 15-30 seconds with 60 second timeout.
    timeout: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CancelAllOrdersAfterResponse {
    /// Current time (RFC3339 timestamp format)
    pub current_time: String,
    /// Trigger time if timer is enabled (RFC3339 timestamp format), null if timer is disabled
    pub trigger_time: Option<String>,
}

impl Response for CancelAllOrdersAfterResponse {}

impl Request for CancelAllOrdersAfter {
    type Response = CancelAllOrdersAfterResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/CancelAllOrdersAfter";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for CancelAllOrdersAfter {}
