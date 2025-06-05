use serde::Deserialize;

use crate::api::trade::OrderBookResponse;

/// Bitgo WebSocket API response
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WsResponse {
    Channel(Channel),
    System(SystemEvent),
    Error(WsError),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "channel", rename_all = "snake_case")]
pub enum Channel {
    #[serde(rename = "level2")]
    OrderBook(OrderBookEvent),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderBookEvent {
    Snapshot(OrderBookResponse),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SystemEvent {
    pub status: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, derive_more::Display, derive_more::Error)]
pub struct WsError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn deserialize_error() {
        let json = r#"{
            "type": "error",
            "message": "Unsupported product"
        }"#;
        let expected = WsResponse::Error(WsError {
            message: "Unsupported product".to_string(),
        });
        let actual: WsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_pong_error() {
        let json = r#"{
            "type": "system",
            "status": "connected"
        }"#;
        let expected = WsResponse::System(SystemEvent {
            status: "connected".to_string(),
        });
        let actual: WsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
