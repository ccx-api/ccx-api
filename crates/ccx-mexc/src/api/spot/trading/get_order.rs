use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::SignedRequest;

use super::Order;

impl Request for GetOrder {
    type Response = Order;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/order";
    const COST: u32 = 2;
}

impl SignedRequest for GetOrder {}

/// Query Order (USER_DATA)
///
/// Check an order's status.
///
/// Weight(IP): 2
///
/// Either orderId or origClientOrderId must be sent.
/// For some historical orders cummulativeQuoteQty will be < 0,
///   meaning the data is not available at this time.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOrder {
    symbol: SmartString,
    order_id: Option<SmartString>,
    orig_client_order_id: Option<SmartString>,
}

impl GetOrder {
    pub fn with_order_id(symbol: SmartString, order_id: SmartString) -> Self {
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
