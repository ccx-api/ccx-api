use serde::Deserialize;
use serde::Serialize;

use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::proto::Response;
use crate::types::rate_limits::RateLimitType;

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

impl Request for GetServerTime {
    type Response = ServerTime;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/time";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl PublicRequest for GetServerTime {}

impl Response for ServerTime {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = GetServerTime {};
        let expected = r#""#;
        let serialized = serde_html_form::to_string(&request).unwrap();
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
