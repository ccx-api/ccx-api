use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::proto::BinanceSpotSigned;
use crate::spot::types::order::MarketSide;
use crate::spot::types::order::OrderStatus;
use crate::spot::types::order::OrderType;
use crate::spot::types::order::SelfTradePreventionMode;
use crate::spot::types::order::TimeInForce;
use crate::spot::types::rate_limits::RateLimitType;
use crate::spot::types::timestamp::BinanceTimestamp;

impl BinanceSpotRequest for GetAllOrders {
    type Response = Vec<Order>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/allOrders";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
}

impl BinanceSpotSigned for GetAllOrders {}

impl BinanceSpotResponse for Vec<Order> {}

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

// [
//   {
//     "symbol": "LTCBTC",
//     "orderId": 1,
//     "orderListId": -1, //Unless it's part of an order list, value will be -1
//     "clientOrderId": "myOrder1",
//     "price": "0.1",
//     "origQty": "1.0",
//     "executedQty": "0.0",
//     "cummulativeQuoteQty": "0.0",
//     "status": "NEW",
//     "timeInForce": "GTC",
//     "type": "LIMIT",
//     "side": "BUY",
//     "stopPrice": "0.0",
//     "icebergQty": "0.0",
//     "time": 1499827319559,
//     "updateTime": 1499827319559,
//     "isWorking": true,
//     "origQuoteOrderQty": "0.000000",
//     "workingTime": 1499827319559,
//     "selfTradePreventionMode": "NONE",
//   }
// ]

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: SmartString,
    pub order_id: u64,
    /// Unless it's part of an order list, value will be -1.
    pub order_list_id: i64,
    pub client_order_id: SmartString<36>,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: MarketSide,
    pub stop_price: Decimal,
    pub iceberg_qty: Decimal,
    pub time: BinanceTimestamp,
    pub update_time: BinanceTimestamp,
    pub is_working: bool,
    pub orig_quote_order_qty: Decimal,
    pub working_time: BinanceTimestamp,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
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
