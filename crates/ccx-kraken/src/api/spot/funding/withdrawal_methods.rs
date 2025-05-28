use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::{AssetClass, AssetName};
use crate::types::deposit::DepositMethodName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Retrieve a list of withdrawal methods available for the user.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WithdrawalMethods {
    /// Filter methods for specific asset
    asset: Option<AssetName>,
    /// Filter methods for specific asset class
    aclass: Option<AssetClass>,
    /// Filter methods for specific network
    network: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WithdrawalMethodsResponse {
    /// Name of asset being withdrawn
    pub asset: AssetName,
    /// Name of the withdrawal method
    pub method: DepositMethodName,
    /// Name of the blockchain or network being withdrawn on
    pub network: String,
    /// Minimum net amount that can be withdrawn right now
    pub minimum: String,
}

impl Response for Vec<WithdrawalMethodsResponse> {}

impl Request for WithdrawalMethods {
    type Response = Vec<WithdrawalMethodsResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/WithdrawMethods";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WithdrawalMethods {}
