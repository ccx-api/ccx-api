use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::spot::PublicTradeInfo;
use crate::proto::PublicRequest;
use crate::proto::Request;

impl Request for GetOldTrades {
    type Response = Vec<PublicTradeInfo>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/historicalTrades";
    const COST: u32 = 5;
}

impl PublicRequest for GetOldTrades {}

/// Old Trade Lookup.
///
/// Get older market trades.
///
/// Weight: 5
///
/// Parameters:
/// * `symbol`
/// * `limit` - default 500; max 1000.
///
/// Data Source: Database
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOldTrades {
    symbol: SmartString,
    limit: Option<u32>,
}

impl GetOldTrades {
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
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = GetOldTrades {
            symbol: "ADAUSDT".into(),
            limit: Some(100),
        };
        let expected = r"symbol=ADAUSDT&limit=100";
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }
}
