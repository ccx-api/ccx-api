use serde::Deserialize;
use serde::Serialize;

use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::proto::BinanceSpotSigned;
use crate::spot::types::rate_limits::RateLimitInterval;
use crate::spot::types::rate_limits::RateLimitType;

impl BinanceSpotRequest for GetUnfilledOrderCount {
    type Response = Vec<UnfilledOrderCount>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/rateLimit/order";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 40)];
}

impl BinanceSpotSigned for GetUnfilledOrderCount {}

impl BinanceSpotResponse for Vec<UnfilledOrderCount> {}

/// [Query Unfilled Order Count (USER_DATA)](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-unfilled-order-count-user_data).
///
/// Displays the user's unfilled order count for all intervals.
///
/// Weight: 40
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetUnfilledOrderCount {}

// [
//   {
//     "rateLimitType": "ORDERS",
//     "interval": "SECOND",
//     "intervalNum": 10,
//     "limit": 50,
//     "count": 0
//   },
//   {
//     "rateLimitType": "ORDERS",
//     "interval": "DAY",
//     "intervalNum": 1,
//     "limit": 160000,
//     "count": 0
//   }
// ]

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnfilledOrderCount {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
    pub count: u32,
}

impl GetUnfilledOrderCount {
    pub fn new() -> Self {
        Self::default()
    }
}
