use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::OrderBookChannel;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct WsRequest {
    /// Optional request id which will be sent back by the server to help you identify which request the server responds to
    pub(super) req_id: Option<i64>,
    /// Channel-dependent fields of request
    #[serde(flatten)]
    pub(super) inner: WsRequestInner,
}

impl WsRequest {
    /// Check if connection to server is still alive.
    pub fn ping() -> Self {
        WsRequestInner::Ping.into()
    }

    /// Periodically notify top bids and asks snapshot with limited levels.
    pub fn order_book(event: WsRequestEvent, payload: OrderBookChannel) -> Self {
        match event {
            WsRequestEvent::Subscribe => WsRequestInner::Subscribe(payload.into()).into(),
            WsRequestEvent::Unsubscribe => WsRequestInner::Unsubscribe(payload.into()).into(),
        }
    }
}

impl From<WsRequestInner> for WsRequest {
    fn from(inner: WsRequestInner) -> Self {
        Self {
            req_id: None,
            inner,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, derive_more::From)]
#[serde(tag = "channel")]
pub enum Channels {
    #[serde(rename = "book")]
    OrderBook(OrderBookChannel),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum WsRequestInner {
    Ping,
    Subscribe(Channels),
    // TODO: possibly the unsubscribe schemas for channels could be different from subscribe
    Unsubscribe(Channels),
}

pub enum WsRequestEvent {
    Subscribe,
    Unsubscribe,
}
