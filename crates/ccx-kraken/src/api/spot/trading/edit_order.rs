use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, NoneAsEmptyString, serde_as, skip_serializing_none};

use crate::prelude::CurrencyPair;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;
use crate::types::trading::{OrderFlag, OrderSide, OrderType, TimeInForce};

use super::{OrderDescription, TxId, Userref};

/// Edit a live order.
///
/// When an order has been successfully modified, the original order will be cancelled
/// and a new order will be created with the adjusted parameters. A new `txid` will be
/// returned in the response.
///
/// Note: The new AmendOrder endpoint resolves the caveats of EditOrder and has additional
/// performance gains. This endpoint has several limitations:
/// - Triggered stop loss or profit take profit orders are not supported
/// - Orders with conditional close terms attached are not supported
/// - Orders where executed volume is greater than newly supplied volume will be rejected
/// - cl_ord_id is not supported
/// - Existing executions are associated with original order and not copied to amended order
/// - Queue position will not be maintained
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct EditOrder {
    #[serde(flatten)]
    id: EditOrderId,
    /// Asset pair id or altname
    pair: CurrencyPair,
    /// Order type
    ordertype: OrderType,
    /// Order direction (buy/sell)
    #[serde(rename = "type")]
    side: OrderSide,
    /// Order quantity in terms of the base asset
    #[serde_as(as = "DisplayFromStr")]
    volume: Decimal,
    /// Price (optional, for limit orders)
    #[serde_as(as = "NoneAsEmptyString")]
    price: Option<Decimal>,
    /// Secondary price (optional, for stop-loss, take-profit, stop-loss-limit, take-profit-limit orders)
    #[serde_as(as = "NoneAsEmptyString")]
    price2: Option<Decimal>,
    /// Order flags (optional)
    oflags: Option<Vec<OrderFlag>>,
    /// Time-in-force of the order
    timeinforce: Option<TimeInForce>,
    /// Validate inputs only. Do not submit order (optional)
    validate: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub enum EditOrderId {
    /// Kraken order identifier
    #[serde(rename = "txid")]
    TxId(TxId),
    /// User reference id
    #[serde(rename = "userref")]
    Userref(Userref),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EditOrderResponse {
    /// Order description info
    pub descr: Option<OrderDescription>,
    /// New Transaction ID (if order was added successfully)
    pub txid: Option<String>,
    /// Original userref if passed with the request
    pub newuserref: Option<String>,
    /// Original userref if passed with the request
    pub olduserref: Option<String>,
    /// Number of orders cancelled (either 0 or 1)
    pub orders_cancelled: Option<i32>,
    /// Original transaction ID
    pub originaltxid: Option<String>,
    /// Status of the order: Ok or Err
    pub status: Option<String>,
    /// Updated volume
    pub volume: Option<String>,
    /// Updated price
    pub price: Option<String>,
    /// Updated price2
    pub price2: Option<String>,
    /// Error message if unsuccessful
    pub error_message: Option<String>,
}

impl Response for EditOrderResponse {}

impl Request for EditOrder {
    type Response = EditOrderResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/EditOrder";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for EditOrder {}
