use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// All Coins' Information (USER_DATA)
///
/// Get information of coins (available for deposit and withdraw) for user.
///
/// Weight(IP): 10
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllCoinsInformation;

impl Request for AllCoinsInformation {
    type Response = Vec<AllCoinsInformationResponse>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/capital/config/getall";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 10)];
}

impl SignedRequest for AllCoinsInformation {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllCoinsInformationResponse {
    pub coin: SmartString,
    pub deposit_all_enable: bool,
    pub free: Decimal,
    pub freeze: Decimal,
    pub ipoable: Decimal,
    pub ipoing: Decimal,
    pub is_legal_money: bool,
    pub locked: Decimal,
    pub name: SmartString,
    pub network_list: Vec<NetworkInformation>,
    pub storage: Decimal,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    pub withdrawing: Decimal,
}

impl Response for AllCoinsInformationResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInformation {
    pub address_regex: SmartString,
    pub coin: SmartString,
    /// Shown only when "depositEnable" is false.
    pub deposit_desc: Option<SmartString>,
    pub deposit_enable: bool,
    pub insert_time: Option<u64>,
    pub is_default: bool,
    pub memo_regex: SmartString,
    /// Min number for balance confirmation.
    pub min_confirm: i32,
    pub name: SmartString,
    pub network: SmartString,
    pub reset_address_status: bool,
    pub special_tips: Option<SmartString>,
    /// Confirmation number for balance unlock.
    pub un_lock_confirm: i32,
    pub update_time: Option<u64>,
    /// Shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<SmartString>,
    pub withdraw_enable: bool,
    pub withdraw_fee: Decimal,
    pub withdraw_min: Decimal,
}
