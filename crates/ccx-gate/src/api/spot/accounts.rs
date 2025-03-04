use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;
use smart_string::SmartString;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

/// # List spot accounts.
///
/// ## Parameters
///
/// * `currency` - Retrieve data of the specified currency
#[derive(Debug, Builder, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accounts {
    pub currency: Option<SmartString>,
}

impl Request for Accounts {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/accounts";
    const COSTS: &'static RateLimitType = &RateLimitType::SpotOther;

    type Response = SmallVec<[SpotAccount; 1]>;
}

impl SignedRequest for Accounts {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotAccount {
    /// Currency detail
    pub currency: SmartString,
    /// Available amount
    pub available: Decimal,
    /// Locked amount, used in trading
    pub locked: Decimal,
}

impl Response for SpotAccount {}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{
    "currency": "ETH",
    "available": "968.8",
    "locked": "0"
  }"#;
        let res: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            SpotAccount {
                currency: "ETH".into(),
                available: dec!(968.8),
                locked: dec!(0),
            }
        );
    }
}
