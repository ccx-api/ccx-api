use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;

/// Withdraw(API)
///
/// Submit a withdraw request.
///
/// Weight(IP): 1
///
/// * withdrawOrderId - client id for withdraw
///
/// If network is not send, return with default network of the coin.
/// You can get network and isDefault in networkList in the response of
///    Get /api/v3/capital/config/getall (HMAC SHA256).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct Withdraw {
    coin: SmartString,
    withdraw_order_id: Option<SmartString>,
    network: Option<SmartString>,
    contract_address: Option<SmartString>,
    address: SmartString,
    memo: Option<SmartString>,
    amount: Decimal,
    remark: Option<SmartString>,
}

impl Request for Withdraw {
    type Response = WithdrawResponse;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/capital/withdraw/apply";
    const COST: u32 = 1;
}

impl SignedRequest for Withdraw {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    pub id: String,
}

impl Response for WithdrawResponse {}
