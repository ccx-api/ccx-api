use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;

use super::TransferType;

/// Withdraw History (supporting network) (USER_DATA)
///
/// Fetch withdraw history.
///
/// Weight(IP): 1
///
/// * startTime - Default: 7 days from current timestamp
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
    limit: Option<u16>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Request for WithdrawHistory {
    type Response = Vec<WithdrawHistoryResponse>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/capital/withdraw/history";
    const COST: u32 = 1;
}

impl SignedRequest for WithdrawHistory {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawHistoryResponse {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    pub coin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub address: String,
    pub amount: Decimal,
    pub transfer_type: TransferType,
    pub status: WithdrawStatus,
    #[serde(default)]
    pub transaction_fee: Decimal,
    pub apply_time: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trans_hash: Option<String>,
    pub update_time: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcoin_id: Option<String>,
}

impl Response for WithdrawHistoryResponse {}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum WithdrawStatus {
    Apply = 1,
    Auditing = 2,
    Wait = 3,
    Processing = 4,
    WaitPackaging = 5,
    WaitConfirm = 6,
    Success = 7,
    Failed = 8,
    Cancel = 9,
    Manual = 10,
}
