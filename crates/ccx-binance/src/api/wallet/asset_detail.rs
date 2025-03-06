use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// [Asset detail](https://developers.binance.com/docs/wallet/asset)
///
/// Weight: 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    asset: SmartString,
}

impl Request for AssetDetail {
    type Response = AssetDetailResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/asset/assetDetail";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for AssetDetail {}

impl AssetDetail {
    pub fn new(asset: impl Into<SmartString>) -> Self {
        Self {
            asset: asset.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetailResponse {
    #[serde(flatten)]
    pub asset: HashMap<SmartString, AssetDetailInfo>,
}

impl Response for AssetDetailResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetailInfo {
    pub min_withdraw_amount: Decimal,
    pub deposit_status: bool,
    pub withdraw_fee: Decimal,
    pub withdraw_status: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_tip: Option<String>,
}
