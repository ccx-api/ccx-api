use serde::{Deserialize, Serialize};

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TickerEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "p")]
    pub price_change: Decimal,
    #[serde(rename = "P")]
    pub price_change_percent: Decimal,
    #[serde(rename = "w")]
    pub weighted_avg_price: Decimal,
    #[serde(rename = "x")]
    pub first_trade: Decimal,
    #[serde(rename = "c")]
    pub last_price: Decimal,
    #[serde(rename = "Q")]
    pub last_qty: Decimal,
    #[serde(rename = "b")]
    pub best_bid_price: Decimal,
    #[serde(rename = "B")]
    pub best_bid_qty: Decimal,
    #[serde(rename = "a")]
    pub best_ask_price: Decimal,
    #[serde(rename = "A")]
    pub best_ask_qty: Decimal,
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
    #[serde(rename = "O")]
    pub stats_open_time: u64,
    #[serde(rename = "C")]
    pub stats_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
}
