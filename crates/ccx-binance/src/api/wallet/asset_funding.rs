use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Response;
use crate::proto::{Request, SignedRequest};
use crate::types::rate_limits::RateLimitType;

/// Funding Wallet (USER_DATA)
///
/// Weight(IP): 1
///
/// * Currently supports querying the following business assets：Binance Pay, Binance Card,
/// Binance Gift Card, Stock Token.zF
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct AssetFunding {
    asset: Option<SmartString>,
    need_btc_valuation: Option<bool>,
}

impl Request for AssetFunding {
    type Response = Vec<AssetFundingResponse>;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/asset/get-funding-asset";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for AssetFunding {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetFundingResponse {
    pub asset: SmartString,
    /// available balance.
    pub free: Decimal,
    /// locked asset.
    pub locked: Decimal,
    /// freeze asset.
    pub freeze: Decimal,
    pub withdrawing: Decimal,
    pub btc_valuation: Decimal,
}

impl Response for AssetFundingResponse {}
