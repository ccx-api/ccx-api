use serde::Deserialize;
use serde::Serialize;

use crate::proto::Request;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Disable Fast Withdraw Switch (USER_DATA)
///
/// Weight(IP): 1
///
/// Caution:
///
/// * This request will disable fastwithdraw switch under your account.
/// * You need to enable "trade" option for the api key which requests this endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableFastWithdraw;

impl Request for DisableFastWithdraw {
    type Response = ();
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/account/disableFastWithdrawSwitch";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for DisableFastWithdraw {}
