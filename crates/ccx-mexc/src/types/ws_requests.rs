use serde::Deserialize;
use serde::Serialize;

pub trait WsMessageRequest {
    type Response: WsMessageResponse;
    const METHOD: &'static str;
}

pub trait WsMessageResponse {}

impl WsMessageResponse for () {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WsMeta<Payload> {
    pub id: Option<WsMessageId>,
    #[serde(flatten)]
    pub payload: Payload,
}

impl<Payload> WsMeta<Payload> {
    pub fn new(id: Option<WsMessageId>, payload: Payload) -> Self {
        Self { id, payload }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct IgnorePayload {}

pub type WsMessageId = u64;
