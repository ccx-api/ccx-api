use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Cancel a recently requested withdrawal, if it has not already been successfully processed.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WithdrawalCancel {
    /// Asset being withdrawn
    asset: AssetName,
    /// Withdrawal reference ID
    refid: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WithdrawalCancelResponse(pub bool);

impl Response for WithdrawalCancelResponse {}

impl Request for WithdrawalCancel {
    type Response = WithdrawalCancelResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/WithdrawCancel";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WithdrawalCancel {}
