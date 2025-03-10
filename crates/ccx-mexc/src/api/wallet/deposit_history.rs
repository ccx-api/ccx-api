use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;

use super::DepositStatus;

/// Deposit History (supporting network) (USER_DATA)
///
/// Fetch deposit history.
///
/// Weight(IP): 1
///
/// * startTime - Default: 7 days from current timestamp
/// * endTime - Default: present timestamp
///
/// * network may not be in the response for old deposit.
/// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
/// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct DepositHistory {
    coin: Option<SmartString>,
    status: Option<DepositStatus>,
    limit: Option<u16>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Request for DepositHistory {
    type Response = Vec<DepositHistoryResponse>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/capital/deposit/hisrec";
    const COST: u32 = 1;
}

impl SignedRequest for DepositHistory {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistoryResponse {
    pub amount: Decimal,
    pub coin: String,
    pub network: SmartString,
    pub status: DepositStatus,
    pub address: String,
    pub tx_id: String,
    pub insert_time: u64,
    pub confirm_times: String,
    pub memo: Option<String>,
    pub unlock_confirm: Option<String>,
}

impl Response for DepositHistoryResponse {}
