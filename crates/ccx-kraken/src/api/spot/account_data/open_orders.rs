use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};
use crate::types::trading::{ClientId, OrderInfo, TxId};

/// Get Open Orders.
///
/// Retrieve information about currently open orders.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct OpenOrders {
    /// Whether or not to include trades related to position in output (default: false)
    trades: Option<bool>,
    /// Restrict results to given user reference id (optional)
    #[serde(flatten)]
    id: Option<ClientId>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpenOrdersResponse {
    /// Array of order info
    pub open: HashMap<TxId, OrderInfo>,
}

impl Response for OpenOrdersResponse {}

impl Request for OpenOrders {
    type Response = OpenOrdersResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/OpenOrders";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::History);
}

impl SignedRequest for OpenOrders {}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn only_required_fields() {
            let open_orders = OpenOrders::builder().build();

            let actual = serde_json::to_value(&open_orders).unwrap();
            let expected = json!({});

            assert_eq!(actual, expected);
        }

        #[test]
        fn with_optional_fields() {
            let open_orders = OpenOrders::builder()
                .trades(true)
                .id(ClientId::Userref(12345))
                .build();

            let actual = serde_json::to_value(&open_orders).unwrap();
            let expected = json!({
                "trades": true,
                "userref": 12345
            });

            assert_eq!(actual, expected);
        }
    }
}
