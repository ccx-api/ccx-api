use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::TimestampSeconds;
use smart_string::SmartString;

use super::create::AccountType;
use super::create::OrderSide;
use super::Order;
use super::OrderStatus;
use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PrivateRequest;
use crate::api::Request;

/// Request list of orders
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct ListOrdersRequest {
    /// Retrieve results with the specified currency pair.
    /// Required for open orders, but optional for finished ones.
    pub currency_pair: SmartString<15>,

    /// List orders based on status.
    pub status: OrderStatus,

    /// Page number of the results.
    pub page: Option<u32>,

    /// Maximum number of records to be returned.
    /// If status is open, the maximum limit is 100.
    pub limit: Option<u32>,

    /// Specify operation account.
    /// Defaults to spot, portfolio, and margin account if not specified.
    /// Set to cross_margin to operate against margin account.
    /// Portfolio margin account must set to cross_margin only.
    pub account: Option<AccountType>,

    /// Start timestamp of the query.
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub from: Option<DateTime<Utc>>,

    /// Time range ending.
    /// Defaults to current time if not specified.
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub to: Option<DateTime<Utc>>,

    /// All bids or asks.
    /// Both included if not specified.
    pub side: Option<OrderSide>,
}

impl ListOrdersRequest {
    pub fn new(currency_pair: &str, status: OrderStatus) -> Self {
        Self {
            currency_pair: currency_pair.into(),
            status,
            page: None,
            limit: None,
            account: None,
            from: None,
            to: None,
            side: None,
        }
    }
}

impl Request for ListOrdersRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = Vec<Order>;
}

impl PrivateRequest for ListOrdersRequest {}
