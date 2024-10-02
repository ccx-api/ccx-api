use serde::Deserialize;
use serde::Serialize;

use crate::spot::proto::BinanceSpotPublic;
use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::rate_limiter::RL_WEIGHT_PER_MINUTE;

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct Ping {}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct Pong {}

impl BinanceSpotRequest for Ping {
    type Response = Pong;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ping";
    const RATE_LIMIT: (&'static str, u32) = (RL_WEIGHT_PER_MINUTE, 1);
}

impl BinanceSpotPublic for Ping {}

impl BinanceSpotResponse for Pong {}

impl Ping {
    pub fn new() -> Self {
        Ping {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = Ping {};
        let expected = r#""#;
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_response_deserialization() {
        let response = r#"{}"#;
        let expected = Pong {};
        let deserialized: Pong = serde_json::from_str(response).unwrap();
        assert_eq!(deserialized, expected);
    }
}
