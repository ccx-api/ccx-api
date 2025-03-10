use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

use super::{DepositStatus, TransferType};

/// Deposit History (supporting network) (USER_DATA)
///
/// Fetch deposit history.
///
/// Weight(IP): 1
///
/// * startTime - Default: 90 days from current timestamp
/// * endTime - Default: present timestamp
///
/// * network may not be in the response for old deposit.
/// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
/// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct DepositHistory {
    include_source: Option<bool>,
    coin: Option<SmartString>,
    status: Option<DepositStatus>,
    offset: Option<u16>,
    limit: Option<u16>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Request for DepositHistory {
    type Response = Vec<DepositHistoryResponse>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/capital/deposit/hisrec";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for DepositHistory {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistoryResponse {
    pub amount: Decimal,
    pub coin: String,
    pub network: String,
    pub status: DepositStatus,
    pub address: String,
    pub address_tag: String,
    /// Returned only when requested with `includeSource` set to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    pub tx_id: String,
    // FIXME decode time, example: "2021-04-29 16:08:00"
    pub insert_time: u64,
    pub transfer_type: TransferType,
    // pub unlock_confirm: String,
    pub confirm_times: String,
}

impl Response for DepositHistoryResponse {}
