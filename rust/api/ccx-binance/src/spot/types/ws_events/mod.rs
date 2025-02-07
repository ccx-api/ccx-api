mod agg_trade;
mod depth;
mod kline;
mod trade;

pub use agg_trade::*;
pub use depth::*;
pub use kline::*;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;
pub use trade::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventType {
    #[serde(rename = "e")]
    pub event_type: SmartString,
}
