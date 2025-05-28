use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::AssetName;
use crate::types::deposit::DepositMethodName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Retrieve (or generate a new) deposit addresses for a particular asset and method.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct DepositAddresses {
    /// Asset being deposited
    asset: AssetName,
    /// Name of the deposit method
    method: DepositMethodName,
    /// Whether or not to generate a new address
    new: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DepositAddressResponse {
    /// Deposit Address.
    pub address: String,
    /// Deposit Address tag.
    pub tag: Option<String>,
    /// Expiration time in unix timestamp, or 0 if not expiring.
    pub expiretm: String,
    /// Whether or not address has ever been used.
    pub new: bool,
}

impl Response for Vec<DepositAddressResponse> {}

impl Request for DepositAddresses {
    type Response = Vec<DepositAddressResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/DepositAddresses";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for DepositAddresses {}
