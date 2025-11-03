use serde::Deserialize;
use serde::Serialize;

use crate::proto::PublicRequest;
use crate::proto::Request;
use crate::proto::Response;

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct Ping {}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct Pong {}

impl Request for Ping {
    type Response = Pong;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ping";
    const COST: u32 = 1;
}

impl PublicRequest for Ping {}

impl Response for Pong {}

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
        let serialized = serde_html_form::to_string(&request).unwrap();
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
