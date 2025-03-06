use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

/// Deposit Address (supporting network) (USER_DATA)
///
/// Fetch deposit address with network.
///
/// Weight(IP): 10
///
/// If network is not send, return with default network of the coin.
/// You can get network and isDefault in networkList in the response of
///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct GetDepositAddress {
    coin: SmartString,
    network: Option<SmartString>,
}

impl Request for GetDepositAddress {
    type Response = GetDepositAddressResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/capital/deposit/address";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 10)];
}

impl SignedRequest for GetDepositAddress {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositAddressResponse {
    pub address: String,
    pub coin: SmartString,
    pub tag: String,
    pub url: String,
}

impl Response for GetDepositAddressResponse {}
