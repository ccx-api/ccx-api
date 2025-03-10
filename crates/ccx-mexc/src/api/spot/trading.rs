mod cancel_order;
mod create_order;
mod get_all_orders;
mod get_order;

pub use crate::types::order::*;
pub use cancel_order::*;
pub use create_order::*;
pub use get_all_orders::*;
pub use get_order::*;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use smart_string::SmartString;

use crate::proto::Response;

// {
//     "symbol": "LTCBTC",
//     "orderId": 1,
//     "orderListId": -1,                 // This field will always have a value of -1 if not an order list.
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
//     "workingTime":1499827319559,
//     "origQuoteOrderQty": "0.000000",
//     "selfTradePreventionMode": "NONE"
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: SmartString,
    pub order_id: String,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    // FIXME make None when < 0.
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: MarketSide,
    pub stop_price: Decimal,
    pub iceberg_qty: Decimal,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: Decimal,
}

impl Response for Order {}
