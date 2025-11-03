use ccx_lib::default;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::proto::Response;

use crate::types::symbols::Symbol;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetExchangeInfo {
    symbol: Option<SmartString>,
    symbols: Option<SmartString>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub timezone: SmartString,
    pub server_time: u64,
    pub symbols: Vec<Symbol>,
}

/// Current exchange trading rules and symbol information.
///
/// Weight: 10
impl Request for GetExchangeInfo {
    type Response = ExchangeInfo;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/exchangeInfo";
    const COST: u32 = 10;
}

impl PublicRequest for GetExchangeInfo {}

impl Response for ExchangeInfo {}

impl GetExchangeInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_symbol(symbol: impl AsRef<str>) -> Self {
        Self {
            symbol: Some(symbol.as_ref().into()),
            ..default()
        }
    }

    pub fn with_symbols(symbols: &[impl AsRef<str>]) -> Self {
        use std::fmt::Write;

        let symbols = if symbols.is_empty() {
            None
        } else {
            let mut list = SmartString::from("[");
            let mut div = "";
            for symbol in symbols {
                write!(list, "{div}{:?}", symbol.as_ref()).unwrap();
                div = ",";
            }
            list.push(']');
            Some(list)
        };
        Self {
            symbols,
            ..default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = GetExchangeInfo::new();
        let expected = r#""#;
        let serialized = serde_html_form::to_string(&request).unwrap();
        assert_eq!(serialized, expected);

        let request = GetExchangeInfo::with_symbol("BNBBTC");
        let expected = r#"symbol=BNBBTC"#;
        let serialized = serde_html_form::to_string(&request).unwrap();
        assert_eq!(serialized, expected);

        let request = GetExchangeInfo::with_symbols(&["BNBBTC", "BTCUSDT"]);
        let expected = r#"symbols=%5B%22BNBBTC%22%2C%22BTCUSDT%22%5D"#;
        let serialized = serde_html_form::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "timezone": "UTC",
            "serverTime": 1565246363776,
            "rateLimits": [],
            "exchangeFilters": [],
            "symbols": []
        }"#;
        let expected = ExchangeInfo {
            timezone: "UTC".into(),
            server_time: 1565246363776,
            symbols: vec![],
        };
        let deserialized: ExchangeInfo = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
