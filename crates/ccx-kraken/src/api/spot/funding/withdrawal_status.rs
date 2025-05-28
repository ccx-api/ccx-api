use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::{AssetClass, AssetName};
use crate::types::deposit::DepositMethodName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Retrieve information about recent withdrawals. Results are sorted by recency, use the cursor parameter to iterate through list of withdrawals (page size equal to value of limit) from newest to oldest.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WithdrawalStatus {
    /// Filter for specific asset being withdrawn
    asset: Option<AssetName>,
    /// Filter for specific asset class being withdrawn
    aclass: Option<AssetClass>,
    /// Filter for specific name of withdrawal method
    method: Option<DepositMethodName>,
    /// Start timestamp, withdrawals created strictly before will not be included in the response
    start: Option<String>,
    /// End timestamp, withdrawals created strictly after will be not be included in the response
    end: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WithdrawalStatusResponse {
    /// Name of withdrawal method
    pub method: DepositMethodName,
    /// Asset class.
    pub aclass: AssetClass,
    /// Asset.
    pub asset: AssetName,
    /// Reference ID.
    pub refid: String,
    /// Method transaction ID.
    pub txid: Option<String>,
    /// Method transaction information.
    pub info: String,
    /// Amount withdrawn
    pub amount: String,
    /// Fees paid.
    pub fee: Option<Decimal>,
    /// Unix timestamp when request made.
    pub time: i64,
    /// Status of deposit.
    pub status: WithdrawalStatusValue,
    /// Addition status properties (if available)
    pub status_prop: Option<WithdrawalStatusProperties>,
    /// Withdrawal key name, as set up on your account
    pub key: String,
}

/// Deposit status according to [IFEX financial transaction states][1].
///
/// [1]: https://github.com/globalcitizen/ifex-protocol/blob/master/draft-ifex-00.txt#L837
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum WithdrawalStatusValue {
    Initial,
    Pending,
    Settled,
    Success,
    Failure,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum WithdrawalStatusProperties {
    /// Cancelation requested.
    #[serde(rename = "cancel-pending")]
    CancelPending,
    /// Withdraw was canceled.
    #[serde(rename = "canceled")]
    Canceled,
    /// Cancelation requested but was denied.
    #[serde(rename = "cancel-denied")]
    CancelDenied,
    /// A return transaction initiated by Kraken; it cannot be canceled.
    #[serde(rename = "return")]
    Return,
    /// Withdrawal is on hold pending review
    #[serde(rename = "onhold")]
    OnHold,
}

impl Response for Vec<WithdrawalStatusResponse> {}

impl Request for WithdrawalStatus {
    type Response = Vec<WithdrawalStatusResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/WithdrawStatus";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WithdrawalStatus {}
