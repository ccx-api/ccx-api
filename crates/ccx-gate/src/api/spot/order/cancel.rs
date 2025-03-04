use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use smart_string::SmartString;

use crate::proto::{Request, SignedRequest};
use crate::types::rate_limits::RateLimitType;

use super::Order;
use super::create::AccountType;

/// Get a single order
///
/// # Endpoint
/// `GET /spot/orders/{order_id}`
///
/// # Description
/// This endpoint retrieves detailed information about a specific order.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, Builder)]
#[builder(on(SmartString<15>, into))]
pub struct CancelOrder {
    /// ID of the order
    #[serde(skip)]
    order_id: SmartString<15>,
    /// Transaction pair to query.
    ///
    /// If you are querying pending order records, this field is required.
    /// If you are querying traded records, this field can be left blank.
    currency_pair: SmartString<15>,
    /// Operation account.
    ///
    /// Defaults to spot, portfolio and margin account if not specified.
    ///
    /// Set to `cross_margin` to operate against margin account.
    /// Portfolio margin account must set to `cross_margin` only
    account: Option<AccountType>,
}

impl CancelOrder {
    pub fn new(id: &str, currency_pair: impl Into<SmartString<15>>) -> Self {
        Self::builder()
            .order_id(id)
            .currency_pair(currency_pair)
            .build()
    }
}

impl Request for CancelOrder {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/orders/{order_id}";
    const COSTS: &'static RateLimitType = &RateLimitType::SpotOrderCancel;

    type Response = Order;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let order_id = &self.order_id;

        format!("/api/v4/spot/orders/{order_id}").into()
    }
}

impl SignedRequest for CancelOrder {}
