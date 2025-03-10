use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::order::MarketSide;
use crate::types::order::OrderType;

impl Request for CreateOrder {
    type Response = NewOrder;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order";
    const COST: u32 = 1;
}

impl SignedRequest for CreateOrder {}

impl Response for NewOrder {}

// --- Test ---

impl Request for CreateOrderTest {
    type Response = NewOrderTest;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order/test";
    const COST: u32 = 1;
}

impl SignedRequest for CreateOrderTest {}

impl Response for NewOrderTest {}

/// New Order (TRADE)
///
/// Send in a new order.
///
/// Weight: 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct CreateOrder {
    symbol: SmartString,
    side: MarketSide,
    r#type: OrderType,
    quantity: Option<Decimal>,
    quote_order_qty: Option<Decimal>,
    price: Option<Decimal>,
    new_client_order_id: Option<SmartString>,
}

/// Test New Order (TRADE)
///
/// Test new order creation and signature/recvWindow long.
/// Creates and validates a new order but does not send it into the matching engine.
///
/// Weight: 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderTest {
    #[serde(flatten)]
    create_order: CreateOrder,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrder {
    pub symbol: SmartString,
    pub order_id: String,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub price: Option<Decimal>,
    pub orig_qty: Option<Decimal>,
    pub r#type: Option<OrderType>,
    pub side: Option<MarketSide>,
    pub transact_time: u64,
}

// --- Test ---

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderTest {}

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
            quantity,
            quote_order_qty,
            price: None,
            new_client_order_id: None,
        }
    }

    pub fn test(self) -> CreateOrderTest {
        CreateOrderTest { create_order: self }
    }
}
