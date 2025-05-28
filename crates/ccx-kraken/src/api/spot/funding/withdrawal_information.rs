use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Get Withdrawal Information
///
/// Retrieve fee information about potential withdrawals for a
/// particular asset, key and amount.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WithdrawalInformation {
    /// Asset being withdrawn
    asset: AssetName,
    /// Withdrawal key name, as set up on your account
    key: String,
    /// Amount to be withdrawn
    amount: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WithdrawalInformationResponse {
    /// Name of the withdrawal method that will be used.
    pub method: String,
    /// Maximum net amount that can be withdrawn right now.
    pub limit: String,
    /// Net amount that will be sent, after fees.
    pub amount: String,
    /// Amount of fees that will be paid
    pub fee: String,
}

impl Response for Vec<WithdrawalInformationResponse> {}

impl Request for WithdrawalInformation {
    type Response = Vec<WithdrawalInformationResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/WithdrawInfo";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WithdrawalInformation {}
