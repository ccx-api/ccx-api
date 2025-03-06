use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Asset Dividend Record (USER_DATA)
///
/// Query asset dividend record.
///
/// Weight(IP): 10
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct AssetDividend {
    asset: Option<SmartString>,
    limit: Option<u16>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Request for AssetDividend {
    type Response = AssetDividendResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/asset/assetDividend";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 10)];
}

impl SignedRequest for AssetDividend {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDividendResponse {
    pub rows: Vec<AssetDividendRow>,
    pub total: u64,
}

impl Response for AssetDividendResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDividendRow {
    id: u64,
    amount: Decimal,
    asset: String,
    div_time: u64,
    en_info: String,
    tran_id: u64,
}
