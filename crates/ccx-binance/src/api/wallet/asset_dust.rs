use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Dust Transfer (USER_DATA)
///
/// Convert dust assets to BNB.
///
/// Weight(UID): 10
///
/// * You need to openEnable Spot & Margin Trading permission
///   for the API Key which requests this endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct AssetDust {
    asset: SmartString,
}

impl AssetDust {
    pub fn new(asset: impl Into<SmartString>) -> Self {
        Self {
            asset: asset.into(),
        }
    }
}

impl Request for AssetDust {
    type Response = AssetDustResponse;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/asset/dust";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 10)];
}

impl SignedRequest for AssetDust {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDustResponse {
    pub total_service_charge: Decimal,
    pub total_transfered: Decimal,
    pub transfer_result: Vec<AssetDustResult>,
}

impl Response for AssetDustResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDustResult {
    pub amount: Decimal,
    pub from_asset: String,
    pub operate_time: u64,
    pub service_charge_amount: Decimal,
    pub tran_id: u64,
    pub transfered_amount: Decimal,
}
