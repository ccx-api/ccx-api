use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Make a withdrawal request.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WithdrawFunds {
    /// Asset being withdrawn
    asset: AssetName,
    /// Withdrawal key name, as set up on your account
    key: String,
    /// Optional, crypto address that can be used to confirm address matches key (will return Invalid withdrawal address error if different)
    address: Option<String>,
    /// Amount to be withdrawn
    amount: String,
    /// Optional, if the processed withdrawal fee is higher than max_fee, withdrawal will fail with EFunding:Max fee exceeded
    max_fee: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WithdrawFundsResponse {
    /// Reference ID of the withdraw.
    pub refid: String,
}

impl Response for WithdrawFundsResponse {}

impl Request for WithdrawFunds {
    type Response = WithdrawFundsResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/Withdraw";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WithdrawFunds {}
