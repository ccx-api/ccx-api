use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::{AssetClass, AssetName};
use crate::types::deposit::DepositMethodName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Get Deposit Methods
///
/// Retrieve methods available for depositing a particular asset.
///
/// * `asset` - Asset being deposited
/// * `aclass` - Asset class being deposited (optional)
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct DepositMethods {
    /// Asset being deposited
    asset: AssetName,
    /// Asset class being deposited (optional)
    aclass: Option<AssetClass>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DepositMethodResponse {
    /// Name of deposit method.
    pub method: DepositMethodName,
    /// Maximum net amount that can be deposited right now, or false if no limit.
    pub limit: DepositMethodLimit,
    /// Amount of fees that will be paid.
    pub fee: Option<Decimal>,
    /// Whether or not method has an address setup fee.
    pub address_setup_fee: Option<Decimal>,
    /// Whether new addresses can be generated for this method..
    pub gen_address: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum DepositMethodLimit {
    Limited(Decimal),
    /// The value expected to be false.
    Unlimited(bool),
}

impl Response for Vec<DepositMethodResponse> {}

impl Request for DepositMethods {
    type Response = Vec<DepositMethodResponse>;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/DepositMethods";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for DepositMethods {}
