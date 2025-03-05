use chrono::Utc;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::order_book::OrderBookRequest;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct WsRequest {
    /// Request time in seconds. Gap between request time and server time must not exceed 60 seconds
    pub(super) time: i64,
    /// Optional request id which will be sent back by the server to help you identify which request the server responds to
    pub(super) id: Option<i64>,
    /// Channel-dependent fields of request
    #[serde(flatten)]
    pub(super) inner: WsRequestInner,
}

impl WsRequest {
    /// Check if connection to server is still alive.
    ///
    /// <https://www.gate.io/docs/developers/apiv4/ws/en/#application-ping-pong>
    pub fn ping() -> Self {
        WsRequestInner::Ping.into()
    }

    /// Periodically notify top bids and asks snapshot with limited levels.
    ///
    /// <https://www.gate.io/docs/developers/apiv4/ws/en/#limited-level-full-order-book-snapshot>
    pub fn order_book(event: WsRequestEvent, payload: OrderBookRequest) -> Self {
        WsRequestInner::OrderBook { event, payload }.into()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WsRequestEvent {
    Subscribe,
    Unsubscribe,
}

impl From<WsRequestInner> for WsRequest {
    fn from(inner: WsRequestInner) -> Self {
        Self {
            time: Utc::now().timestamp(),
            id: None,
            inner,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "channel")]
pub enum WsRequestInner {
    #[serde(rename = "spot.ping")]
    Ping,
    #[serde(rename = "spot.order_book")]
    OrderBook {
        event: WsRequestEvent,
        payload: OrderBookRequest,
    },
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::WsRequestEvent;
    use crate::api_ws::order_book::Interval;
    use crate::api_ws::order_book::Level;
    use crate::api_ws::order_book::OrderBookRequest;
    use crate::api_ws::request::WsRequest;

    #[test]
    fn serialize_ping() {
        let mut request = WsRequest::ping();
        request.time = 1724168425;
        assert_eq!(
            r#"{"time":1724168425,"channel":"spot.ping"}"#,
            serde_json::to_string(&request).unwrap()
        );
    }

    #[test]
    fn serialize_order_book() {
        let expected = r#"{
  "time": 1724168425,
  "channel": "spot.order_book",
  "event": "subscribe",
  "payload": [
    "BTC_USDT",
    "20",
    "100ms"
  ]
}"#;
        let mut request = WsRequest::order_book(
            WsRequestEvent::Subscribe,
            OrderBookRequest {
                pair: "BTC_USDT".into(),
                level: Level::L20,
                interval: Interval::Ms100,
            },
        );
        request.time = 1724168425;
        assert_eq!(expected, serde_json::to_string_pretty(&request).unwrap());
    }
}
