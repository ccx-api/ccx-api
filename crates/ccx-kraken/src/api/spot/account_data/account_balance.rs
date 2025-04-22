use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Get Account Balance.
///
/// Retrieve all cash balances, net of pending withdrawals.
#[derive(Serialize, Debug)]
pub struct AccountBalance {}

impl AccountBalance {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Deserialize, Debug)]
pub struct AccountBalanceResponse {
    #[serde(flatten)]
    pub balance: HashMap<AssetName, Decimal>,
}

impl Response for AccountBalanceResponse {}

impl Request for AccountBalance {
    type Response = AccountBalanceResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/Balance";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for AccountBalance {}
