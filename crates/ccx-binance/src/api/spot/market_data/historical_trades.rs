use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::spot::PublicTradeInfo;
use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::types::rate_limits::RateLimitType;

impl Request for GetOldTrades {
    type Response = Vec<PublicTradeInfo>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/historicalTrades";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 25)];
}

impl PublicRequest for GetOldTrades {}

/// [Old trade lookup](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#old-trade-lookup)
///
/// Get older trades.
///
/// Weight: 25
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetOldTrades {
    symbol: SmartString,
    limit: Option<u32>,
    from_id: Option<u64>,
}

impl GetOldTrades {
    pub fn new(symbol: SmartString) -> Self {
        Self {
            symbol,
            limit: None,
            from_id: None,
        }
    }

    /// Default 500; max 1000.
    pub fn with_limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    /// Trade id to fetch from. Default gets most recent trades.
    pub fn with_from_id(self, from_id: u64) -> Self {
        Self {
            from_id: Some(from_id),
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
            from_id: Some(123456),
        };
        let expected = r"symbol=ADAUSDT&limit=100&fromId=123456";
        let serialized = serde_html_form::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }
}
