use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Account Status (USER_DATA)
///
/// Fetch account status detail.
///
/// Weight(IP): 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus;

impl Request for AccountStatus {
    type Response = AccountStatusResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/account/status";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for AccountStatus {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatusResponse {
    pub data: SmartString,
}

impl Response for AccountStatusResponse {}
