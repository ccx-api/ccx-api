use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// DustLog(USER_DATA)
///
/// Weight(IP): 1
///
/// * Only return last 100 records
/// * Only return records after 2020/12/01
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct AssetDribblet {
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Request for AssetDribblet {
    type Response = AssetDribbletResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/asset/dribblet";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for AssetDribblet {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDribbletResponse {
    pub total: Option<u64>,
    pub user_asset_dribblets: Vec<UserAssetDribblet>,
}

impl Response for AssetDribbletResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetDribblet {
    pub operate_time: u64,
    pub total_transfered_amount: Decimal,
    pub total_service_charge_amount: Decimal,
    pub trans_id: u64,
    pub user_asset_dribblet_details: Vec<UserAssetDribbletDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetDribbletDetails {
    pub trans_id: u64,
    pub service_charge_amount: Decimal,
    pub amount: Decimal,
    pub operate_time: u64,
    pub transfered_amount: Decimal,
    pub from_asset: String,
}
