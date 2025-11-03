use ccx_lib::default;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::proto::Response;
use crate::types::filters::Filter;
use crate::types::rate_limits::RateLimit;
use crate::types::rate_limits::RateLimitType;
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
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<Filter>,
    pub symbols: Vec<Symbol>,
    // pub sors: Vec<Sor>,
}

// {
//   "timezone": "UTC",
//   "serverTime": 1565246363776,
//   "rateLimits": [
//     {
//       // These are defined in the `ENUM definitions` section under `Rate Limiters (rateLimitType)`.
//       // All limits are optional
//     }
//   ],
//   "exchangeFilters": [
//     // These are the defined filters in the `Filters` section.
//     // All filters are optional.
//   ],
//   "symbols": [
//     {
//       "symbol": "ETHBTC",
//       "status": "TRADING",
//       "baseAsset": "ETH",
//       "baseAssetPrecision": 8,
//       "quoteAsset": "BTC",
//       "quotePrecision": 8, // will be removed in future api versions (v4+)
//       "quoteAssetPrecision": 8,
//       "baseCommissionPrecision": 8,
//       "quoteCommissionPrecision": 8,
//       "orderTypes": [
//         "LIMIT",
//         "LIMIT_MAKER",
//         "MARKET",
//         "STOP_LOSS",
//         "STOP_LOSS_LIMIT",
//         "TAKE_PROFIT",
//         "TAKE_PROFIT_LIMIT"
//       ],
//       "icebergAllowed": true,
//       "ocoAllowed": true,
//       "otoAllowed": true,
//       "quoteOrderQtyMarketAllowed": true,
//       "allowTrailingStop": false,
//       "cancelReplaceAllowed":false,
//       "isSpotTradingAllowed": true,
//       "isMarginTradingAllowed": true,
//       "filters": [
//         // These are defined in the Filters section.
//         // All filters are optional
//       ],
//       "permissions": [],
//       "permissionSets": [
//         [
//           "SPOT",
//           "MARGIN"
//         ]
//       ],
//       "defaultSelfTradePreventionMode": "NONE",
//       "allowedSelfTradePreventionModes": [
//         "NONE"
//       ]
//     }
//   ],
//   // Optional field. Present only when SOR is available.
//   // https://github.com/binance/binance-spot-api-docs/blob/master/faqs/sor_faq.md
//   "sors": [
//     {
//       "baseAsset": "BTC",
//       "symbols": [
//         "BTCUSDT",
//         "BTCUSDC"
//       ]
//     }
//   ]
// }

impl Request for GetExchangeInfo {
    type Response = ExchangeInfo;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/exchangeInfo";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
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
            rate_limits: vec![],
            exchange_filters: vec![],
            symbols: vec![],
        };
        let deserialized: ExchangeInfo = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
