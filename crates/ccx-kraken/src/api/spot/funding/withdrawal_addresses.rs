use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::{AssetClass, AssetName};
use crate::types::deposit::DepositMethodName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Retrieve a list of withdrawal addresses available for the user.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WithdrawalAddresses {
    /// Filter addresses for specific asset
    asset: Option<AssetName>,
    /// Filter addresses for specific asset class
    aclass: Option<AssetClass>,
    /// Filter addresses for specific method
    method: Option<DepositMethodName>,
    /// Find address for by withdrawal key name, as set up on your account
    key: Option<String>,
    /// Filter by verification status of the withdrawal address. Withdrawal addresses successfully completing email confirmation will have a verification status of true.
    verified: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WithdrawalAddressResponse {
    /// Withdrawal address
    pub address: String,
    /// Name of asset being withdrawn
    pub asset: AssetName,
    /// Name of the withdrawal method
    pub method: String,
    /// Withdrawal key name, as set up on your account
    pub key: String,
    /// Deposit Address tag.
    pub tag: Option<String>,
    /// Verification status of withdrawal address
    pub verified: bool,
}

impl Response for Vec<WithdrawalAddressResponse> {}

impl Request for WithdrawalAddresses {
    type Response = Vec<WithdrawalAddressResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/WithdrawAddresses";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WithdrawalAddresses {}
