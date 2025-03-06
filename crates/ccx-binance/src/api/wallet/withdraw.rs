use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Withdraw(SAPI)
///
/// Submit a withdraw request.
///
/// Weight(IP): 1
///
/// * withdrawOrderId - client id for withdraw
/// * addressTag - Secondary address identifier for coins like XRP,XMR etc.
/// * transactionFeeFlag - When making internal transfer, true for returning the fee
///     to the destination account; false for returning the fee back to the departure account.
///     Default false.
/// * name - Description of the address. Space in name should be encoded into %20.
///
/// If network is not send, return with default network of the coin.
/// You can get network and isDefault in networkList in the response of
///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct Withdraw {
    coin: SmartString,
    withdraw_order_id: Option<SmartString>,
    network: Option<SmartString>,
    address: SmartString,
    address_tag: Option<SmartString>,
    amount: Decimal,
    transaction_fee_flag: Option<bool>,
    name: Option<SmartString>,
}

impl Request for Withdraw {
    type Response = WithdrawResponse;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/capital/withdraw/apply";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for Withdraw {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    pub id: String,
}

impl Response for WithdrawResponse {}
