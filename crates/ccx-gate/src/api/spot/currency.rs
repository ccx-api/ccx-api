use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::{PublicRequest, Request, Response};

/// List all currencies' details
///
/// `GET /spot/currencies`
///
/// Currency has two forms:
/// * Only currency name, e.g., `BTC`, `USDT`
/// * `<currency>_<chain>`, e.g., `HT_ETH`
///
/// ## Parameters
/// None
#[derive(Debug, Clone, Serialize)]
pub struct AllCurrencies;

impl Request for AllCurrencies {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/currencies";
    type Response = Vec<CurrencyResponse>;
}

impl PublicRequest for AllCurrencies {}

/// Get details of a specific currency
///
/// `GET /spot/currencies/{name}`
///
/// Get details of a specific currency
/// ## Parameters
/// * `currency`
#[derive(Debug, Clone, Serialize)]
pub struct Currency {
    #[serde(skip)]
    currency: String,
}

impl Currency {
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
        }
    }
}

impl Request for Currency {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/currencies/{currency}";
    type Response = CurrencyResponse;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let currency = &self.currency;
        format!("/api/v4/spot/currencies/{currency}").into()
    }
}

impl PublicRequest for Currency {}

/// Represents the details of a currency.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CurrencyResponse {
    /// Currency symbol
    pub currency: SmartString,
    /// Whether currency is de-listed
    pub delisted: bool,
    /// Whether currency's withdrawal is disabled
    pub withdraw_disabled: bool,
    /// Whether currency's withdrawal is delayed
    pub withdraw_delayed: bool,
    /// Whether currency's deposit is disabled
    pub deposit_disabled: bool,
    /// Whether currency's trading is disabled
    pub trade_disabled: bool,
    /// Fixed fee rate. Only for fixed rate currencies, not valid for normal currencies
    #[serde(default, with = "crate::util::maybe_str")]
    pub fixed_rate: Option<Decimal>,
    /// Chain of currency
    #[serde(default)]
    pub chain: Option<SmartString>,
    // TODO:
    // name: String
    // pub chains: Vec<_>
}

impl Response for CurrencyResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_currency() {
        let json = r#"{
            "currency": "GT",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "chain": "GT"
        }"#;

        let expected = CurrencyResponse {
            currency: "GT".into(),
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: None,
            chain: Some("GT".into()),
        };
        assert_eq!(
            serde_json::from_str::<CurrencyResponse>(json).unwrap(),
            expected
        );
    }
}
