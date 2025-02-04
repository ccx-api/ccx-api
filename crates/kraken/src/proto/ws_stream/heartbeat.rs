use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Heartbeat {
    pub event: HeartbeatEvent,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatEvent {
    Heartbeat,
}
