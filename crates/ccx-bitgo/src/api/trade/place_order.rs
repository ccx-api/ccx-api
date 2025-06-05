use chrono::{DateTime, Utc};
use macro_rules_attribute::apply;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::prelude::Product;
use crate::proto::{Request, SignedRequest};
use crate::types::derive::Request;
use crate::types::order::{FundingType, Order, OrderSide};
use crate::types::rate_limits::RateLimitType;

/// Limit order request
#[apply(Request)]
pub struct LimitOrder {
    limit_price: Decimal,
    /// Duration of the limit order in minutes. For orders that are good till cancelled, skip duration and use time_in_force=GTC.
    duration: Option<u32>,
    /// Time in force for the order. GTC (Good Till Cancelled) is the only allowed value.
    time_in_force: Option<String>,
}

/// TWAP order request
#[apply(Request)]
pub struct TwapOrder {
    /// The limit price for the order (optional)
    limit_price: Option<Decimal>,
    /// Duration of the TWAP order in minutes (required)
    duration: i32,
    /// Interval of the time-sliced TWAP order in minutes
    /// Only required if `isTimeSliced` is `true` in parameters
    interval: Option<i32>,
    /// Time when the order is scheduled to be executed
    scheduled_date: Option<DateTime<Utc>>,
    /// Additional parameters for the TWAP order
    parameters: Option<TwapParameters>,
}

/// SteadyPace order request
#[apply(Request)]
pub struct SteadyPaceOrder {
    pub limit_price: Option<Decimal>,
    /// Time when the order is scheduled to be executed
    pub scheduled_date: Option<DateTime<Utc>>,
    /// Additional parameters for the SteadyPace order
    pub parameters: Option<SteadyPaceParameters>,
}

/// TWAP order parameters
#[apply(Request)]
pub struct TwapParameters {
    /// Whether the order uses time slicing
    /// - If set to true, the order will be executed using a time-sliced strategy.
    /// - If set to false, the order will be executed using a regular TWAP strategy without time slicing.
    /// - If not specified, the default behavior uses a regular TWAP strategy without time slicing.
    pub is_time_sliced: Option<bool>,
    /// Controls how strictly the TWAP algorithm adheres to the target fill progression
    /// - It is optional but can be provided for a regular TWAP strategy.
    /// - Default value is `standard`.
    /// Allowed values:
    /// - narrow
    /// - standard
    /// - wide
    pub bounds_control: Option<String>,
}

/// SteadyPace order parameters
#[apply(Request)]
pub struct SteadyPaceParameters {
    /// The interval for the SteadyPace order, specified in conjunction with the interval unit.
    /// This is a required field.
    pub interval: i32,

    /// The unit of time for the interval. Defaults to "minute".
    /// Allowed values:
    /// - second
    /// - minute
    /// - hour
    /// This is a required field.
    pub interval_unit: String,

    /// The size of each sub-order in the SteadyPace order.
    /// This is a required field.
    pub sub_order_size: String,

    /// Optional degree of randomization for sub-order sizes.
    /// Accepts a decimal value rounded to two decimal places between 0 and 1,
    /// representing the variation in the size of each sub-order.
    /// For example, a value of 0.20 indicates a 20% variance.
    pub variance: Option<f64>,
}

/// Enum to represent different order types for the request
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlaceOrderType {
    Market,
    Limit(LimitOrder),
    Twap(TwapOrder),
    SteadyPace(SteadyPaceOrder),
}

#[apply(Request)]
pub struct PlaceOrder {
    /// The id of the trading account to retrieve
    #[serde(skip)]
    account_id: String,
    /// Custom order ID. This must be a unique ID associated with an order
    /// and cannot be the same across multiple requests.
    /// Max length: <= 256 characters
    client_order_id: Option<String>,
    /// Product name e.g. BTC-USD
    product: Product,
    /// The order request details
    #[serde(flatten)]
    order_type: PlaceOrderType,
    /// The funding type of the order
    funding_type: FundingType,
    /// Side of the order (buy or sell)
    side: OrderSide,
    /// The specified quantity
    quantity: Decimal,
    /// The quantity currency must be in quote currency for buy unless a limit price is specified, in which case buy orders can only be placed in the base currency. For sell, the quantity currency must be in base. e.g. If product is BTC-USD, the base currency will be BTC.
    quantity_currency: String,
}

impl Request for PlaceOrder {
    type Response = Order;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;

        format!("/api/prime/trading/v1/accounts/{account_id}/orders").into()
    }
}

impl SignedRequest for PlaceOrder {}
