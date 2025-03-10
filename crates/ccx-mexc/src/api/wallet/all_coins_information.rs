use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;

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
    const ENDPOINT: &'static str = "/api/v3/capital/config/getall";
    const COST: u32 = 10;
}

impl SignedRequest for AllCoinsInformation {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllCoinsInformationResponse {
    pub coin: SmartString,
    pub name: SmartString,
    pub network_list: Vec<NetworkInformation>,
}

impl Response for AllCoinsInformationResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInformation {
    pub coin: SmartString,
    /// Shown only when "depositEnable" is false.
    pub deposit_desc: Option<SmartString>,
    pub deposit_enable: bool,
    pub deposit_tips: Option<SmartString>,
    /// Min number for balance confirmation.
    pub min_confirm: i32,
    pub name: SmartString,
    #[serde(rename = "netWork")]
    pub network: SmartString,
    /// Shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<SmartString>,
    pub withdraw_enable: bool,
    pub withdraw_fee: Decimal,
    pub withdraw_min: Decimal,
    pub withdraw_max: String,
    pub same_address: bool,
    pub contract: Option<SmartString>,
    pub withdraw_tip: Option<SmartString>,
}
