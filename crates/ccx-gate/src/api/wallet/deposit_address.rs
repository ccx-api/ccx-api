use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositAddress {
    currency: SmartString,
}

impl DepositAddress {
    pub fn new(currency: impl Into<SmartString>) -> Self {
        Self {
            currency: currency.into(),
        }
    }
}

/// # Generate currency deposit address
///
/// Generate currency deposit address
///
/// ## Parameters
///
/// * `currency` - Currency name
impl Request for DepositAddress {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/wallet/deposit_address";
    const COSTS: &'static RateLimitType = &RateLimitType::WalletOther;

    type Response = DepositAddressResponse;
}

impl SignedRequest for DepositAddress {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositAddressResponse {
    pub currency: SmartString,
    pub address: SmartString,
    pub multichain_addresses: Vec<DepositAddressMultichainAddress>,
    pub min_deposit_amount: SmartString,
    pub min_confirms: Option<SmartString>,
}

impl Response for DepositAddressResponse {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositAddressMultichainAddress {
    pub chain: SmartString,
    pub address: SmartString,
    pub payment_id: SmartString,
    pub payment_name: SmartString,
    pub obtain_failed: u32,
    pub min_confirms: Option<u32>,
}
