use chrono::{DateTime, Utc};
use macro_rules_attribute::apply;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::coin::Coin;
use super::derive::Response;
use super::product::Product;

/// Order status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// Order is pending open
    PendingOpen,
    /// Order is open and working
    Open,
    /// Order has been completed (filled)
    Completed,
    /// Order is pending cancellation
    PendingCancel,
    /// Order has been canceled
    Canceled,
    /// Order has an error
    Error,
    /// Order is scheduled for future execution
    Scheduled,
}

/// Order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
}

/// Order side
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingType {
    /// Buy order
    Margin,
    /// Sell order
    Funded,
}

/// Order information
#[apply(Response)]
pub struct Order {
    /// Unique identifier for the order
    pub id: Uuid,
    /// Account ID associated with the order
    pub account_id: String,
    /// Client-supplied order id
    pub client_order_id: Option<String>,
    /// Time when the order was created
    pub creation_date: DateTime<Utc>,
    /// Time when the order is scheduled to be executed
    pub scheduled_date: Option<DateTime<Utc>>,
    /// Time when the order had its last fill
    pub last_fill_date: Option<DateTime<Utc>>,
    /// Time when the order was completed
    pub completion_date: Option<DateTime<Utc>>,
    /// Time when the order was settled
    pub settle_date: Option<DateTime<Utc>>,
    /// Type of order (market, limit)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Funding type of the order (margin, funded)
    pub funding_type: FundingType,
    /// Status of the order
    pub status: OrderStatus,
    /// Product name e.g. BTC-USD
    pub product: Product,
    /// Side of the order (buy or sell)
    pub side: OrderSide,
    /// The specified quantity
    pub quantity: Decimal,
    /// The specified quantity currency
    pub quantity_currency: Coin,
    /// The base quantity that was filled
    pub filled_quantity: Decimal,
    /// The quote quantity that was filled
    pub filled_quote_quantity: Decimal,
    /// The average price at which the order was filled
    pub average_price: Decimal,
    /// The limit price for limit orders
    pub limit_price: Option<Decimal>,
    /// Duration of the order in minutes
    pub duration: Option<i32>,
    /// Interval length of the TWAP order in minutes
    pub twap_interval: Option<i32>,
    /// Reason for order cancellation
    pub reason: Option<String>,
    /// Additional parameters for the order
    pub parameters: Option<OrderParameters>,
}

/// Additional parameters for an order
#[apply(Response)]
pub struct OrderParameters {
    /// Whether the order uses time slicing
    pub is_time_sliced: Option<bool>,
    /// Controls how strictly the TWAP algorithm adheres to the target fill progression
    pub bounds_control: Option<String>,
    /// The interval for the SteadyPace order
    pub interval: Option<i32>,
    /// The unit of time for the interval
    pub interval_unit: Option<String>,
    /// The size of each sub-order in the SteadyPace order
    pub sub_order_size: Option<String>,
    /// Degree of randomization for sub-order sizes
    pub variance: Option<f64>,
}
