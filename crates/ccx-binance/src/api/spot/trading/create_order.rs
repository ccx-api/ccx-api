use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::spot::Discount;
use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::order::MarketSide;
use crate::types::order::OrderStatus;
use crate::types::order::OrderType;
use crate::types::order::SelfTradePreventionMode;
use crate::types::order::TimeInForce;
use crate::types::rate_limits::RateLimitType;
use crate::types::timestamp::BinanceTimestamp;

// --- Ack ---

impl Request for CreateOrderAck {
    type Response = NewOrderAck;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for CreateOrderAck {}

impl Response for NewOrderAck {}

// --- Result ---

impl Request for CreateOrderResult {
    type Response = NewOrderResult;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for CreateOrderResult {}

impl Response for NewOrderResult {}

// --- Full ---

impl Request for CreateOrderFull {
    type Response = NewOrderFull;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for CreateOrderFull {}

impl Response for NewOrderFull {}

// --- Test ---

impl Request for CreateOrderTest {
    type Response = NewOrderTest;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order/test";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for CreateOrderTest {}

impl Response for NewOrderTest {}

// --- Test with rates ---

impl Request for CreateOrderTestWithRates {
    type Response = NewOrderTestWithRates;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order/test";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
}

impl SignedRequest for CreateOrderTestWithRates {}

impl Response for NewOrderTestWithRates {}

// --- --- ---

// Name	Type	Mandatory	Description
// symbol	STRING	YES
// side	ENUM	YES	Please see Enums for supported values.
// type	ENUM	YES	Please see Enums for supported values
// timeInForce	ENUM	NO	Please see Enums for supported values.
// quantity	DECIMAL	NO
// quoteOrderQty	DECIMAL	NO
// price	DECIMAL	NO
// newClientOrderId	STRING	NO	A unique id among open orders. Automatically generated if not sent.
// Orders with the same newClientOrderID can be accepted only when the previous one is filled, otherwise the order will be rejected.
// strategyId	LONG	NO
// strategyType	INT	NO	The value cannot be less than 1000000.
// stopPrice	DECIMAL	NO	Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
// trailingDelta	LONG	NO	Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
// icebergQty	DECIMAL	NO	Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
// newOrderRespType	ENUM	NO	Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
// selfTradePreventionMode	ENUM	NO	The allowed enums is dependent on what is configured on the symbol. The possible supported values are: STP Modes.

/// [New order (TRADE)](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-trade).
///
/// Send in a new order.
///
/// Weight: 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrder {
    symbol: SmartString,
    side: MarketSide,
    r#type: OrderType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    /// A unique id among open orders. Automatically generated if not sent.
    /// Orders with the same newClientOrderID can be accepted only when the previous one is filled, otherwise the order will be rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<SmartString<36>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    strategy_id: Option<u64>,
    /// The value cannot be less than 1000000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    strategy_type: Option<u32>,
    /// Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    /// Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<u64>,
    /// Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    /// The allowed enums is dependent on what is configured on the symbol. The possible supported values are: STP Modes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderAck {
    #[serde(flatten)]
    create_order: CreateOrder,

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK
    new_order_resp_type: NewOrderResponseType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResult {
    #[serde(flatten)]
    create_order: CreateOrder,

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK
    new_order_resp_type: NewOrderResponseType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderFull {
    #[serde(flatten)]
    create_order: CreateOrder,

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK
    new_order_resp_type: NewOrderResponseType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum NewOrderResponseType {
    Ack,
    Result,
    Full,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderTest {
    #[serde(flatten)]
    create_order: CreateOrder,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderTestWithRates {
    #[serde(flatten)]
    create_order: CreateOrder,

    compute_commission_rates: bool,
}

// --- Ack ---

// {
//   "symbol": "BTCUSDT",
//   "orderId": 28,
//   "orderListId": -1, // Unless it's part of an order list, value will be -1
//   "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
//   "transactTime": 1507725176595
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderAck {
    pub symbol: SmartString,
    pub order_id: u64,
    /// Unless it's part of an order list, value will be -1.
    pub order_list_id: i64,
    pub client_order_id: SmartString<36>,
    pub transact_time: BinanceTimestamp,
}

// --- Result ---

// {
//   "symbol": "BTCUSDT",
//   "orderId": 28,
//   "orderListId": -1, // Unless it's part of an order list, value will be -1
//   "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
//   "transactTime": 1507725176595,
//   "price": "0.00000000",
//   "origQty": "10.00000000",
//   "executedQty": "10.00000000",
//   "origQuoteOrderQty": "0.000000",
//   "cummulativeQuoteQty": "10.00000000",
//   "status": "FILLED",
//   "timeInForce": "GTC",
//   "type": "MARKET",
//   "side": "SELL",
//   "workingTime": 1507725176595,
//   "selfTradePreventionMode": "NONE"
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResult {
    pub symbol: SmartString,
    pub order_id: u64,
    /// Unless it's part of an order list, value will be -1.
    pub order_list_id: i64,
    pub client_order_id: SmartString<36>,
    pub transact_time: BinanceTimestamp,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub orig_quote_order_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: MarketSide,
    pub working_time: BinanceTimestamp,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

// --- Full ---

// {
//   "symbol": "BTCUSDT",
//   "orderId": 28,
//   "orderListId": -1, // Unless it's part of an order list, value will be -1
//   "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
//   "transactTime": 1507725176595,
//   "price": "0.00000000",
//   "origQty": "10.00000000",
//   "executedQty": "10.00000000",
//   "origQuoteOrderQty": "0.000000",
//   "cummulativeQuoteQty": "10.00000000",
//   "status": "FILLED",
//   "timeInForce": "GTC",
//   "type": "MARKET",
//   "side": "SELL",
//   "workingTime": 1507725176595,
//   "selfTradePreventionMode": "NONE",
//   "fills": [
//     {
//       "price": "4000.00000000",
//       "qty": "1.00000000",
//       "commission": "4.00000000",
//       "commissionAsset": "USDT",
//       "tradeId": 56
//     },
//     {
//       "price": "3999.00000000",
//       "qty": "5.00000000",
//       "commission": "19.99500000",
//       "commissionAsset": "USDT",
//       "tradeId": 57
//     },
//     {
//       "price": "3998.00000000",
//       "qty": "2.00000000",
//       "commission": "7.99600000",
//       "commissionAsset": "USDT",
//       "tradeId": 58
//     },
//     {
//       "price": "3997.00000000",
//       "qty": "1.00000000",
//       "commission": "3.99700000",
//       "commissionAsset": "USDT",
//       "tradeId": 59
//     },
//     {
//       "price": "3995.00000000",
//       "qty": "1.00000000",
//       "commission": "3.99500000",
//       "commissionAsset": "USDT",
//       "tradeId": 60
//     }
//   ]
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderFull {
    pub symbol: SmartString,
    pub order_id: u64,
    /// Unless it's part of an order list, value will be -1.
    pub order_list_id: i64,
    pub client_order_id: SmartString<36>,
    pub transact_time: BinanceTimestamp,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub orig_quote_order_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: MarketSide,
    pub working_time: BinanceTimestamp,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub price: Decimal,
    pub qty: Decimal,
    pub commission: Decimal,
    pub commission_asset: SmartString,
    pub trade_id: u64,
}

// --- Test ---

// {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderTest {}

// --- Test with rates ---

// {
//   "standardCommissionForOrder": {  //Standard commission rates on trades from the order.
//     "maker": "0.00000112",
//     "taker": "0.00000114",
//   },
//   "taxCommissionForOrder": {       //Tax commission rates for trades from the order.
//     "maker": "0.00000112",
//     "taker": "0.00000114",
//   },
//   "discount": {                    //Discount on standard commissions when paying in BNB.
//     "enabledForAccount": true,
//     "enabledForSymbol": true,
//     "discountAsset": "BNB",
//     "discount": "0.25000000"       //Standard commission is reduced by this rate when paying commission in BNB.
//   }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderTestWithRates {
    standard_commission_for_order: NewOrderCommission,
    tax_commission_for_order: NewOrderCommission,
    discount: Discount,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderCommission {
    maker: Decimal,
    taker: Decimal,
}

// --- --- ---

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuantitySide {
    Base,
    Quote,
}

impl CreateOrder {
    pub fn new_market(
        symbol: SmartString,
        market_side: MarketSide,
        quantity: Decimal,
        quantity_side: QuantitySide,
    ) -> Self {
        let (quantity, quote_order_qty) = match quantity_side {
            QuantitySide::Base => (Some(quantity), None),
            QuantitySide::Quote => (None, Some(quantity)),
        };
        Self {
            symbol,
            side: market_side,
            r#type: OrderType::Market,
            time_in_force: None,
            quantity,
            quote_order_qty,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            self_trade_prevention_mode: None,
        }
    }

    pub fn ack(self) -> CreateOrderAck {
        CreateOrderAck {
            create_order: self,
            new_order_resp_type: NewOrderResponseType::Ack,
        }
    }

    pub fn result(self) -> CreateOrderResult {
        CreateOrderResult {
            create_order: self,
            new_order_resp_type: NewOrderResponseType::Result,
        }
    }

    pub fn full(self) -> CreateOrderFull {
        CreateOrderFull {
            create_order: self,
            new_order_resp_type: NewOrderResponseType::Full,
        }
    }

    pub fn test(self) -> CreateOrderTest {
        CreateOrderTest { create_order: self }
    }
}

impl CreateOrderTest {
    pub fn with_rates(self) -> CreateOrderTestWithRates {
        CreateOrderTestWithRates {
            create_order: self.create_order,
            compute_commission_rates: true,
        }
    }
}
