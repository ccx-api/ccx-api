use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

use super::TransferType;

/// Withdraw History (supporting network) (USER_DATA)
///
/// Fetch withdraw history.
///
/// Weight(IP): 1
///
/// * startTime - Default: 90 days from current timestamp
/// * endTime - Default: present timestamp
///
/// * network may not be in the response for old withdraw.
/// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
/// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct WithdrawHistory {
    coin: Option<SmartString>,
    status: Option<WithdrawStatus>,
    offset: Option<u16>,
    limit: Option<u16>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Request for WithdrawHistory {
    type Response = Vec<WithdrawHistoryResponse>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/capital/withdraw/history";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl SignedRequest for WithdrawHistory {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawHistoryResponse {
    pub address: String,
    pub amount: Decimal,
    #[serde(default)]
    pub transaction_fee: Decimal,
    // FIXME decode time, example: "2021-04-29 16:08:00"
    pub apply_time: String,
    pub coin: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub transfer_type: TransferType,
    pub status: WithdrawStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
}

impl Response for WithdrawHistoryResponse {}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum WithdrawStatus {
    EmailSent = 0,
    Cancelled = 1,
    AwaitingApproval = 2,
    Rejected = 3,
    Processing = 4,
    Failure = 5,
    Completed = 6,
}
