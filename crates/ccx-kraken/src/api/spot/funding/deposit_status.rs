use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::{AssetClass, AssetName};
use crate::types::deposit::DepositMethodName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Get Status of Recent Deposits
///
/// Retrieve information about recent deposits. Any deposits initiated
/// in the past 90 days will be included in the response, up to a
/// maximum of 25 results, sorted by recency.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct DepositStatus {
    /// Filter for specific asset being deposited
    asset: Option<AssetName>,
    /// Filter for specific asset class being deposited
    aclass: Option<AssetClass>,
    /// Filter for specific name of deposit method
    method: Option<DepositMethodName>,
    /// Start timestamp, deposits created strictly before will not be included in the response
    start: Option<String>,
    /// End timestamp, deposits created strictly after will be not be included in the response
    end: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DepositStatusResponse {
    /// Name of deposit method.
    pub method: DepositMethodName,
    /// Asset class.
    pub aclass: AssetClass,
    /// Asset.
    pub asset: AssetName,
    /// Reference ID.
    pub refid: String,
    /// Method transaction ID.
    pub txid: String,
    /// Method transaction information.
    pub info: String,
    /// Amount deposited.
    pub amount: Decimal,
    /// Fees paid (if any).
    #[serde(default)]
    pub fee: Option<Decimal>,
    /// Unix timestamp when request made.
    pub time: i64,
    /// Status of deposit.
    pub status: DepositStatusValue,
    /// Addition status properties (if available)
    pub status_prop: Option<DepositStatusProperties>,
    /// Client sending transaction id(s) for deposits that credit with a
    /// sweeping transaction
    #[serde(default)]
    pub originators: Vec<String>,
}

/// Deposit status according to [IFEX financial transaction states][1].
///
/// [1]: https://github.com/globalcitizen/ifex-protocol/blob/master/draft-ifex-00.txt#L837
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DepositStatusValue {
    Initial,
    Pending,
    Settled,
    Success,
    Failure,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DepositStatusProperties {
    /// A return transaction initiated by Kraken; it cannot be canceled.
    #[serde(rename = "return")]
    Return,

    /// Deposit is on hold pending review
    #[serde(rename = "onhold")]
    OnHold,
}

impl Response for Vec<DepositStatusResponse> {}

impl Request for DepositStatus {
    type Response = Vec<DepositStatusResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/DepositStatus";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for DepositStatus {}
