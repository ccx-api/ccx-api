use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::proto::Response;

impl Request for GetRecentTrades {
    type Response = Vec<PublicTradeInfo>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/trades";
    const COST: u32 = 5;
}

impl PublicRequest for GetRecentTrades {}

impl Response for Vec<PublicTradeInfo> {}

/// Recent trades list.
///
/// Get recent trades.
///
/// Weight: 5
///
/// Parameters:
/// * `symbol`
/// * `limit` - default 500; max 1000.
///
/// Data Source: Memory
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetRecentTrades {
    symbol: SmartString,
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradeInfo {
    // TODO: I saw only null values for the id so far...
    pub id: Option<u64>,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TradeType {
    BID,
    ASK,
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
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_response_deserialization() {
        let response = r#"[
            {
                "id": null,
                "price": "23",
                "qty": "0.478468",
                "quoteQty": "11.004764",
                "time": 1640830579240,
                "isBuyerMaker": true,
                "isBestMatch": true
            }
        ]"#;
        let expected = vec![PublicTradeInfo {
            id: None,
            price: dec!(23),
            qty: dec!(0.478468),
            quote_qty: dec!(11.004764),
            time: 1640830579240,
            is_buyer_maker: true,
            is_best_match: true,
        }];
        let deserialized: Vec<PublicTradeInfo> = serde_json::from_str(response).unwrap();
        assert_eq!(deserialized, expected);
    }
}
