use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;

use crate::types::symbols::SymbolPermission;

impl Request for GetAccountInfo {
    type Response = AccountInfo;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/account";
    const COST: u32 = 10;
}

impl SignedRequest for GetAccountInfo {}

impl Response for AccountInfo {}

/// Account Information (USER_DATA).
///
/// Get current account information.
///
/// Weight(IP): 10
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfo;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub update_time: Option<u64>,
    pub account_type: AccountType,
    pub balances: Vec<Balance>,
    // FIXME choose apropriate kind of permission.
    pub permissions: Vec<SymbolPermission>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum AccountType {
    #[serde(rename = "SPOT")]
    Spot,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: SmartString,
    pub free: Decimal,
    pub locked: Decimal,
}
