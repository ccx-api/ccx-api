use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::proto::Response;
use crate::types::rate_limits::RateLimitType;
use crate::types::timestamp::BinanceTimestamp;

impl Request for GetRecentTrades {
    type Response = Vec<PublicTradeInfo>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/trades";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 25)];
}

impl PublicRequest for GetRecentTrades {}

impl Response for Vec<PublicTradeInfo> {}

/// [Recent trades list](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#recent-trades-list)
///
/// Get recent trades.
///
/// Weight: 25
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetRecentTrades {
    symbol: SmartString,
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradeInfo {
    pub id: u64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: BinanceTimestamp,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

impl GetRecentTrades {
    pub fn new(symbol: SmartString) -> Self {
        Self {
            symbol,
            limit: None,
        }
    }

    /// Default 500; max 1000.
    pub fn with_limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = GetRecentTrades {
            symbol: "ADAUSDT".into(),
            limit: Some(100),
        };
        let expected = r"symbol=ADAUSDT&limit=100";
        let serialized = serde_html_form::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_response_deserialization() {
        let response = r#"[
            {
                "id": 28457,
                "price": "4.00000100",
                "qty": "12.00000000",
                "quoteQty": "48.000012",
                "time": 1499865549590,
                "isBuyerMaker": true,
                "isBestMatch": true
            }
        ]"#;
        let expected = vec![PublicTradeInfo {
            id: 28457,
            price: dec!(4.000001),
            qty: dec!(12),
            quote_qty: dec!(48.000012),
            time: BinanceTimestamp::from_epoch_millis(1499865549590).unwrap(),
            is_buyer_maker: true,
            is_best_match: true,
        }];
        let deserialized: Vec<PublicTradeInfo> = serde_json::from_str(response).unwrap();
        assert_eq!(deserialized, expected);
    }
}
