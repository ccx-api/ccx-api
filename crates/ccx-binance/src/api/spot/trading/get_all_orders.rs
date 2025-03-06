use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;
use crate::types::timestamp::BinanceTimestamp;

use super::Order;

impl Request for GetAllOrders {
    type Response = Vec<Order>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/allOrders";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
}

impl SignedRequest for GetAllOrders {}

// symbol	STRING	YES
// orderId	LONG	NO
// startTime	LONG	NO
// endTime	LONG	NO
// limit	INT	NO	Default 500; max 1000.

/// [All orders (USER_DATA)](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#all-orders-user_data).
///
/// Get all account orders; active, canceled, or filled.
///
/// Weight: 20
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrders {
    symbol: SmartString,
    order_id: Option<u64>,
    start_time: Option<BinanceTimestamp>,
    end_time: Option<BinanceTimestamp>,
    limit: Option<u32>,
}

impl GetAllOrders {
    pub fn new(symbol: SmartString) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn new_with_time(
        symbol: SmartString,
        start_time: Option<BinanceTimestamp>,
        end_time: Option<BinanceTimestamp>,
    ) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time,
            end_time,
            limit: None,
        }
    }

    /// * limit — Default 500; max 1000.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}
