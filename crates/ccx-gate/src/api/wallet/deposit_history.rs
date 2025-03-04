use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

/// # Retrieve deposit records
///
/// *Record time range cannot exceed 30 days*
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Builder)]
#[builder(on(SmartString, into))]
pub struct DepositHistory {
    currency: Option<SmartString>,
    from: Option<SmartString>,
    to: Option<SmartString>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Request for DepositHistory {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/wallet/deposits";
    const COSTS: &'static RateLimitType = &RateLimitType::WalletOther;

    type Response = Vec<DepositHistoryResponse>;
}

impl SignedRequest for DepositHistory {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositHistoryResponse {
    pub id: SmartString,
    pub timestamp: SmartString,
    pub withdraw_order_id: Option<SmartString>,
    pub currency: SmartString,
    pub address: SmartString,
    pub txid: SmartString,
    pub amount: SmartString,
    pub memo: SmartString,
    pub status: DepositStatus,
    pub chain: SmartString,
}

impl Response for DepositHistoryResponse {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DepositStatus {
    /// Done
    Done,
    /// Cancelled
    Cancel,
    /// Requesting
    Request,
    /// Pending manual approval
    Manual,
    /// GateCode operation
    Bcode,
    /// Pending confirm after sending
    Extpend,
    /// Pending confirm when fail
    Fail,
    /// Invalid order
    Invalid,
    /// Verifying
    Verify,
    /// Processing
    Proces,
    /// Pending
    Pend,
    /// Required manual approval
    Dmove,
    /// The order is automatically split due to large amount
    Splitpend,
}
