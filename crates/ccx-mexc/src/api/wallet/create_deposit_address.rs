use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;

/// Deposit Address (supporting network) (USER_DATA)
///
/// Fetch deposit address with network.
///
/// Weight(IP): 1
///
/// If network is not send, return with default network of the coin.
/// You can get network and isDefault in networkList in the response of
///    Get /api/v3/capital/config/getall (HMAC SHA256).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct CreateDepositAddress {
    coin: SmartString,
    network: SmartString,
}

impl Request for CreateDepositAddress {
    type Response = CreateDepositAddressResponse;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v3/capital/deposit/address";
    const COST: u32 = 1;
}

impl SignedRequest for CreateDepositAddress {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepositAddressResponse {
    pub address: String,
    pub coin: SmartString,
    pub memo: Option<String>,
    pub network: SmartString,
}

impl Response for CreateDepositAddressResponse {}
