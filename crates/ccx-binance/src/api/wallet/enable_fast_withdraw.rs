use serde::Deserialize;
use serde::Serialize;

use crate::proto::Request;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Enable Fast Withdraw Switch (USER_DATA)
///
/// Weight(IP): 1
///
/// This request will enable fastwithdraw switch under your account.
/// You need to enable "trade" option for the api key which requests this endpoint.
/// When Fast Withdraw Switch is on, transferring funds to a Binance account will be done
///   instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableFastWithdraw;

impl Request for EnableFastWithdraw {
    type Response = ();
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/account/enableFastWithdrawSwitch";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for EnableFastWithdraw {}
