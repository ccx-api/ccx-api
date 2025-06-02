use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};
use crate::types::trading::{ClientId, OrderInfo, TxId};

/// Get Closed Orders.
///
/// Retrieve information about orders that have been closed (filled or cancelled).
/// 50 results are returned at a time, the most recent by default.
///
/// Note: If an order's tx ID is given for `start` or `end` time, the order's opening time (`opentm`) is used.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct ClosedOrders {
    /// Whether or not to include trades related to position in output (default: false)
    trades: Option<bool>,
    /// Restrict results to given user reference id (optional)
    #[serde(flatten)]
    id: Option<ClientId>,
    /// Starting unix timestamp or order tx ID of results (optional)
    start: Option<u64>,
    /// Ending unix timestamp or order tx ID of results (optional)
    end: Option<u64>,
    /// Result offset for pagination (optional)
    ofs: Option<u32>,
    /// Which time to use to search (optional)
    closetime: Option<CloseTime>,
    /// Whether or not to consolidate trades by individual taker trades
    consolidate_taker: Option<bool>,
    /// Whether or not to include page count in result (true is much faster for users with many closed orders)
    without_count: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClosedOrdersResponse {
    /// Number of closed orders available
    pub count: u32,
    /// Array of closed order info
    pub closed: HashMap<TxId, OrderInfo>,
}

/// Which time to use to search.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CloseTime {
    Both,
    Open,
    Close,
}

impl Response for ClosedOrdersResponse {}

impl Request for ClosedOrders {
    type Response = ClosedOrdersResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/ClosedOrders";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::History);
}

impl SignedRequest for ClosedOrders {}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn only_required_fields() {
            let closed_orders = ClosedOrders::builder().build();

            let actual = serde_json::to_value(&closed_orders).unwrap();
            let expected = json!({});

            assert_eq!(actual, expected);
        }

        #[test]
        fn with_optional_fields() {
            let closed_orders = ClosedOrders::builder()
                .trades(true)
                .id(ClientId::Userref(12345))
                .start(1234567890u32)
                .end(1234567900u32)
                .ofs(10u32)
                .closetime(CloseTime::Both)
                .build();

            let actual = serde_json::to_value(&closed_orders).unwrap();
            let expected = json!({
                "trades": true,
                "userref": 12345,
                "start": 1234567890,
                "end": 1234567900,
                "ofs": 10,
                "closetime": "both"
            });

            assert_eq!(actual, expected);
        }

        #[test]
        fn with_closetime_variants() {
            let closed_orders_open = ClosedOrders::builder().closetime(CloseTime::Open).build();

            let actual = serde_json::to_value(&closed_orders_open).unwrap();
            let expected = json!({
                "closetime": "open"
            });

            assert_eq!(actual, expected);

            let closed_orders_close = ClosedOrders::builder().closetime(CloseTime::Close).build();

            let actual = serde_json::to_value(&closed_orders_close).unwrap();
            let expected = json!({
                "closetime": "close"
            });

            assert_eq!(actual, expected);
        }
    }
}
