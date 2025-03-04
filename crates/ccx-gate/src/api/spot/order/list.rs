use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::TimestampSeconds;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use smart_string::SmartString;

use crate::proto::{Request, SignedRequest};
use crate::types::rate_limits::RateLimitType;

use super::Order;
use super::OrderStatus;
use super::create::AccountType;
use super::create::OrderSide;

/// List orders
///
/// # Endpoint
/// `GET /spot/orders`
///
/// # Description
/// Spot, portfolio and margin orders are returned by default.
/// If cross margin orders are needed, `account` must be set to `cross_margin`
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(on(SmartString<15>, into))]
pub struct ListOrders {
    /// Retrieve results with the specified currency pair.
    /// Required for open orders, but optional for finished ones.
    currency_pair: SmartString<15>,

    /// List orders based on status.
    status: OrderStatus,

    /// Page number of the results.
    page: Option<u32>,

    /// Maximum number of records to be returned.
    /// If status is open, the maximum limit is 100.
    limit: Option<u32>,

    /// Specify operation account.
    /// Defaults to spot, portfolio, and margin account if not specified.
    /// Set to cross_margin to operate against margin account.
    /// Portfolio margin account must set to cross_margin only.
    account: Option<AccountType>,

    /// Start timestamp of the query.
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    from: Option<DateTime<Utc>>,

    /// Time range ending.
    /// Defaults to current time if not specified.
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    to: Option<DateTime<Utc>>,

    /// All bids or asks.
    /// Both included if not specified.
    side: Option<OrderSide>,
}

impl ListOrders {
    pub fn new(currency_pair: &str, status: OrderStatus) -> Self {
        Self::builder()
            .currency_pair(currency_pair)
            .status(status)
            .build()
    }
}

impl Request for ListOrders {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/orders";
    const COSTS: &'static RateLimitType = &RateLimitType::SpotOther;

    type Response = Vec<Order>;
}

impl SignedRequest for ListOrders {}
