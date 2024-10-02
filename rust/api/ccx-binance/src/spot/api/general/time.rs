use serde::Deserialize;
use serde::Serialize;

use crate::spot::proto::BinanceSpotPublic;
use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::rate_limiter::RL_WEIGHT_PER_MINUTE;

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct GetServerTime {}

impl GetServerTime {
    pub fn new() -> Self {
        GetServerTime {}
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

impl BinanceSpotRequest for GetServerTime {
    type Response = ServerTime;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/time";
    const RATE_LIMIT: (&'static str, u32) = (RL_WEIGHT_PER_MINUTE, 1);
}

impl BinanceSpotPublic for GetServerTime {}

impl BinanceSpotResponse for ServerTime {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = GetServerTime {};
        let expected = r#""#;
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{"serverTime": 1499827319559}"#;
        let expected = ServerTime {
            server_time: 1499827319559,
        };
        let deserialized: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
