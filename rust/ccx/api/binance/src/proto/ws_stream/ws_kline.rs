use serde::{Deserialize, Serialize};

use crate::Atom;
use crate::ChartInterval;
use crate::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct WSKline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "i")]
    pub interval: ChartInterval,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "V")]
    pub active_buy_volume: Decimal,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: Decimal,
    #[serde(skip, rename = "B")]
    pub ignore: (),
}
