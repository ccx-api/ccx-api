use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Pong {
    pub reqid: u64,
    pub event: String,
}
