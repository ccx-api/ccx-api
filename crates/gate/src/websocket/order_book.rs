use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeSeq;
use smart_string::SmartString;

use crate::api::spot::PriceAndAmount;

/// Order book WebSocket request payload
#[derive(Debug, Clone)]
pub struct OrderBookRequest {
    pub pair: SmartString<12>,
    pub level: Level,
    pub interval: Interval,
}

impl Serialize for OrderBookRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.pair)?;
        seq.serialize_element(&self.level)?;
        seq.serialize_element(&self.interval)?;
        seq.end()
    }
}

/// Order book level
#[derive(Debug, Serialize, Clone, Copy)]
pub enum Level {
    /// Level 5
    #[serde(rename = "5")]
    L5,
    /// Level 10
    #[serde(rename = "10")]
    L10,
    /// Level 20
    #[serde(rename = "20")]
    L20,
    /// Level 50
    #[serde(rename = "50")]
    L50,
    /// Level 100
    #[serde(rename = "100")]
    L100,
}

/// Order book update interval
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Interval {
    /// 100 ms
    #[serde(rename = "100ms")]
    Ms100,
    /// 1000 ms
    #[serde(rename = "1000ms")]
    Ms1000,
}

/// Represents a snapshot of the order book.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct OrderBookSnapshot {
    /// Order book update time in milliseconds.
    #[serde(rename = "t")]
    pub update_time_ms: i64,

    /// Order book update ID of this snapshot.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,

    /// Currency pair.
    #[serde(rename = "s")]
    pub currency_pair: SmartString<12>,

    /// Top level bids in the current snapshot, sorted by price from high to low.
    pub bids: Vec<PriceAndAmount>,

    /// Top level asks in the current snapshot, sorted by price from low to high.
    pub asks: Vec<PriceAndAmount>,
}
