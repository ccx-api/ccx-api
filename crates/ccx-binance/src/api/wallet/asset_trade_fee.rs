use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Trade Fee (USER_DATA)
///
/// Fetch trade fee
///
/// Weight(IP): 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct AssetTradeFee {
    symbol: Option<SmartString>,
}

impl Request for AssetTradeFee {
    type Response = Vec<AssetTradeFeeResponse>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/asset/tradeFee";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for AssetTradeFee {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetTradeFeeResponse {
    pub symbol: String,
    pub maker_commission: Decimal,
    pub taker_commission: Decimal,
}

impl Response for AssetTradeFeeResponse {}
