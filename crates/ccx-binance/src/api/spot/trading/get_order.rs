use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

use super::Order;

impl Request for GetOrder {
    type Response = Order;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/order";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 4)];
}

impl SignedRequest for GetOrder {}

// symbol	STRING	YES
// orderId	LONG	NO
// origClientOrderId	STRING	NO

/// [Query order (USER_DATA)](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#query-order-user_data).
///
/// Check an order's status.
///
/// Weight: 4
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOrder {
    symbol: SmartString,
    order_id: Option<u64>,
    orig_client_order_id: Option<SmartString>,
}

impl GetOrder {
    pub fn with_order_id(symbol: SmartString, order_id: u64) -> Self {
        Self {
            symbol,
            order_id: Some(order_id),
            orig_client_order_id: None,
        }
    }

    pub fn with_orig_client_order_id(
        symbol: SmartString,
        orig_client_order_id: impl Into<SmartString>,
    ) -> Self {
        Self {
            symbol,
            order_id: None,
            orig_client_order_id: Some(orig_client_order_id.into()),
        }
    }
}
