use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;
use std::collections::HashMap;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Builder)]
pub struct WithdrawStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<SmartString>,
}

impl WithdrawStatus {
    pub fn new() -> Self {
        Self { currency: None }
    }

    pub fn with_currency(currency: impl Into<SmartString>) -> Self {
        Self {
            currency: Some(currency.into()),
        }
    }
}

/// # Query withdrawal status
///
/// Query withdrawal status
///
/// API operations are not supported for tokens with low liquidity or extremely low value.
/// Please use the Web or App interface to query and process.
///
/// ## Parameters
///
/// * `currency` - Query by specified currency name (optional)
impl Request for WithdrawStatus {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/wallet/withdraw_status";
    const COSTS: &'static RateLimitType = &RateLimitType::WalletOther;

    type Response = WithdrawStatusResponse;
}

impl SignedRequest for WithdrawStatus {}

pub type WithdrawStatusResponse = Vec<WithdrawStatusItem>;

impl Response for WithdrawStatusResponse {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawStatusItem {
    pub currency: SmartString,
    pub name: SmartString,
    pub name_cn: SmartString,
    pub deposit: SmartString,
    pub withdraw_percent: SmartString,
    pub withdraw_fix: Decimal,
    pub withdraw_day_limit: Decimal,
    pub withdraw_amount_mini: Decimal,
    pub withdraw_day_limit_remain: Decimal,
    pub withdraw_eachtime_limit: Decimal,
    #[serde(default)]
    pub withdraw_fix_on_chains: HashMap<SmartString, Decimal>,
    #[serde(default)]
    pub withdraw_percent_on_chains: HashMap<SmartString, SmartString>,
}
