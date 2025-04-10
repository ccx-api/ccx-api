use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use smart_string::SmartString;

use crate::proto::{Request, SignedRequest};
use crate::types::currency_pair::CurrencyPair;
use crate::types::rate_limits::RateLimitType;

use super::Order;

/// Create an order
///
/// # Endpoint
/// `POST /spot/orders`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[cfg_attr(test, derive(PartialEq))]
#[builder(on(CurrencyPair, into))]
#[non_exhaustive]
pub struct CreateOrder {
    /// Currency pair (e.g., BTC_USDT).
    pub currency_pair: CurrencyPair,

    /// Account type (e.g., spot, margin, unified, cross_margin). Defaults to `spot`
    pub account: Option<AccountType>,

    /// Order side (buy or sell).
    pub side: OrderSide,

    /// The amount of the order. For limit orders, it refers to the base currency.
    pub amount: Decimal,

    /// The price of the order. Required for limit orders. Optional.
    // TODO: price is required when using Limit order type (by default). Improve usage on type level
    pub price: Option<Decimal>,

    /// Time in force for the order (e.g., gtc, ioc, poc, fok). Optional.
    time_in_force: Option<TimeInForce>,

    /// The amount to display for iceberg orders. Null or 0 for normal orders. Optional.
    pub iceberg: Option<Decimal>,

    /// Enables automatic borrowing if the balance is insufficient. Used in margin or cross margin accounts. Optional.
    pub auto_borrow: Option<bool>,

    /// Enables or disables automatic repayment for auto-borrowed loans in cross margin orders. Optional.
    pub auto_repay: Option<bool>,

    /// Self-trading prevention action. Determines the strategy for preventing self-trades. Optional.
    #[serde(rename = "stp_act")]
    pub stp_action: Option<StpAction>,

    /// Processing mode. Specifies the response detail level. Defaults to `FULL`.
    pub action_mode: Option<ActionMode>,

    /// User-defined information. If provided, must follow specific formatting rules. Optional.
    pub text: Option<SmartString<30>>,

    /// Order type (limit or market). Optional.
    #[serde(rename = "type")]
    pub order_type: Option<OrderType>,
}

impl CreateOrder {
    pub fn new(currency_pair: &str, side: OrderSide, amount: Decimal) -> Self {
        Self::builder()
            .currency_pair(currency_pair)
            .side(side)
            .amount(amount)
            .build()
    }
}

impl Request for CreateOrder {
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v4/spot/orders";
    const COSTS: &'static RateLimitType = &RateLimitType::SpotOrderCreateChange;

    type Response = Order;
}

impl SignedRequest for CreateOrder {}

/// Represents the different types of accounts.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    /// Spot account.
    Spot,

    /// Margin account.
    Margin,

    /// Unified account.
    Unified,

    /// Cross margin account.
    CrossMargin,
}

/// Represents the side of the order.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(PartialEq))]
pub enum OrderSide {
    /// Buy order.
    Buy,

    /// Sell order.
    Sell,
}

/// Represents the type of the order.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(PartialEq))]
pub enum OrderType {
    /// Limit order.
    Limit,

    /// Market order.
    Market,
}

/// Represents the time in force options for the order.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TimeInForce {
    /// The order remains active until it is fully filled or canceled.
    #[serde(rename = "gtc")]
    GoodTillCancelled,

    /// The order must be filled immediately or it will be canceled.
    #[serde(rename = "ioc")]
    ImmediateOrCancelled,

    /// The order is post-only and will not take liquidity.
    #[serde(rename = "poc")]
    PendingOrCancelled,

    /// The order must be completely filled or it will be canceled.
    #[serde(rename = "fok")]
    FillOrKill,
}

/// Represents self-trading prevention actions.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StpAction {
    /// Cancel newest: Cancel new orders and keep the old ones.
    #[serde(rename = "cn")]
    CancelNewest,

    /// Cancel oldest: Cancel old orders and keep the new ones.
    #[serde(rename = "co")]
    CancelOldest,

    /// Cancel both: Both old and new orders will be canceled.
    #[serde(rename = "cb")]
    CancelBoth,

    /// No action ('-'): No self-trading prevention action.
    #[serde(rename = "-")]
    NoAction,
}

/// Represents the processing mode for placing an order.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ActionMode {
    /// Asynchronous mode: Only returns key order fields.
    #[serde(rename = "ACK")]
    Asynchronous,

    /// Result mode: No clearing information is returned.
    #[serde(rename = "RESULT")]
    Result,

    /// Full mode (default): Full response is returned.
    #[serde(rename = "FULL")]
    Full,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use serde_json;
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn serialize_create_order_request_simple() {
        // Create an example instance of CreateOrder
        let order = CreateOrder {
            text: Some("t-abc123".into()),
            order_type: Some(OrderType::Limit),
            account: Some(AccountType::Unified),
            price: Some(dec!(65000)),
            time_in_force: Some(TimeInForce::GoodTillCancelled),
            iceberg: Some(dec!(0)),
            ..CreateOrder::new("BTC_USDT", OrderSide::Buy, dec!(0.001))
        };
        // Serialize the CreateOrder instance to a JSON string
        let serialized = serde_json::to_string_pretty(&order).expect("Serialization failed");

        // Expected JSON string
        let expected = r#"{
  "currency_pair": "BTC_USDT",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0",
  "text": "t-abc123",
  "type": "limit"
}"#;

        // Assert that the serialized JSON matches the expected JSON
        assert_eq!(expected, serialized);
    }

    #[test]
    fn serialize_create_order_request_all_fields() {
        // Create an example instance of CreateOrder
        let order = CreateOrder {
            currency_pair: "BTC_USDT".into(),
            account: Some(AccountType::Spot),
            side: OrderSide::Buy,
            amount: dec!(0.5),        // Represents 0.5
            price: Some(dec!(30000)), // Represents 30000
            time_in_force: Some(TimeInForce::GoodTillCancelled),
            iceberg: Some(dec!(0.1)), // Represents 0.1
            auto_borrow: Some(true),
            auto_repay: Some(false),
            stp_action: Some(StpAction::CancelNewest),
            action_mode: Some(ActionMode::Full),
            text: Some("t-order123".into()),
            order_type: Some(OrderType::Limit),
        };

        // Serialize the CreateOrder instance to a JSON string
        let serialized = serde_json::to_string_pretty(&order).expect("Serialization failed");

        // Expected JSON string
        let expected = r#"{
  "currency_pair": "BTC_USDT",
  "account": "spot",
  "side": "buy",
  "amount": "0.5",
  "price": "30000",
  "time_in_force": "gtc",
  "iceberg": "0.1",
  "auto_borrow": true,
  "auto_repay": false,
  "stp_act": "cn",
  "action_mode": "FULL",
  "text": "t-order123",
  "type": "limit"
}"#;

        // Assert that the serialized JSON matches the expected JSON
        assert_eq!(expected, serialized);
    }
}
