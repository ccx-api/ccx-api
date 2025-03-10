use serde::Deserialize;
use serde::Serialize;
use smart_string::PascalString;

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

    pub fn new_int(id: i64, payload: Payload) -> Self {
        Self::new(Some(WsMessageId::Int(id)), payload)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct IgnorePayload {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WsMessageId {
    Int(i64),
    String(PascalString<36>),
}
