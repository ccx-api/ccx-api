use serde::Serialize;
use serde_with::skip_serializing_none;
use smart_string::SmartString;

use super::Order;
use super::create::AccountType;
use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PrivateRequest;
use crate::api::Request;

/// Params for getting an order
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderParams {
    /// Transaction pair to query.
    ///
    /// If you are querying pending order records, this field is required.
    /// If you are querying traded records, this field can be left blank.
    pub currency_pair: Option<SmartString<15>>,
    /// Operation account.
    ///
    /// Defaults to spot, portfolio and margin account if not specified.
    ///
    /// Set to `cross_margin` to operate against margin account.
    /// Portfolio margin account must set to `cross_margin` only
    pub account: Option<AccountType>,
}

impl Request for GetOrderParams {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = Order;
}

impl PrivateRequest for GetOrderParams {}
