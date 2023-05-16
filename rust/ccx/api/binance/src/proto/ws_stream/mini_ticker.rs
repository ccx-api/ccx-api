use serde::{Deserialize, Serialize};

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct MiniTickerEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
}
