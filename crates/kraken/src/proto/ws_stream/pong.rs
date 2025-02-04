use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Pong {
    pub reqid: u64,
    pub event: String,
}
