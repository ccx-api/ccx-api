use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Account API Trading Status (USER_DATA)
///
/// Fetch account api trading status detail.
///
/// Weight(IP): 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradingStatus;

impl Request for AccountTradingStatus {
    type Response = AccountTradingStatusResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/account/apiTradingStatus";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for AccountTradingStatus {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradingStatusResponse {
    pub data: AccountTradingStatusData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradingStatusData {
    pub is_locked: bool,
    pub planned_recover_time: u64,
    pub trigger_condition: HashMap<String, u64>,
    pub update_time: u64,
}

impl Response for AccountTradingStatusResponse {}
