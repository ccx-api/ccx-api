use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::spot::proto::BinanceSpotPublic;
use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::types::rate_limits::RateLimitType;

impl BinanceSpotRequest for GetOrderBook {
    type Response = OrderBook;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/depth";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 250)];
}

impl BinanceSpotPublic for GetOrderBook {}

impl BinanceSpotResponse for OrderBook {}

/// [Order book](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#order-book)
///
/// Weight:
///
/// Adjusted based on the limit:
///
/// Limit | Request Weight
/// --- | ---
/// 1-100 | 5
/// 101-500 | 25
/// 501-1000 | 50
/// 1001-5000 | 250
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderBook {
    symbol: SmartString,
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: i64,
    pub bids: Vec<OrderBookRow>,
    pub asks: Vec<OrderBookRow>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookRow {
    pub price: Decimal,
    pub qty: Decimal,
}

impl GetOrderBook {
    pub fn new(symbol: SmartString) -> Self {
        Self {
            symbol,
            limit: None,
        }
    }

    /// Default 100; max 5000.
    /// If limit > 5000. then the response will truncate to 5000.
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
        let request = GetOrderBook {
            symbol: "ADAUSDT".into(),
            limit: Some(100),
        };
        let expected = r"symbol=ADAUSDT&limit=100";
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_response_deserialization() {
        let response = r#"{
            "lastUpdateId": 1027024,
            "bids": [
                [
                    "4.00000000",
                    "431.00000000"
                ]
            ],
            "asks": [
                [
                    "4.00000200",
                    "12.00000000"
                ]
            ]
        }"#;
        let expected = OrderBook {
            last_update_id: 1027024,
            bids: vec![OrderBookRow {
                price: dec!(4),
                qty: dec!(431),
            }],
            asks: vec![OrderBookRow {
                price: dec!(4.000002),
                qty: dec!(12),
            }],
        };
        let deserialized: OrderBook = serde_json::from_str(response).unwrap();
        assert_eq!(deserialized, expected);
    }
}
