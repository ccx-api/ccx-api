use ccx_api_lib::Decimal;
use serde::Deserialize;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Deserialize, Clone)]
pub enum TickerSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TickerMessage {
    pub sequence: u64,
    pub product_id: Atom,
    pub price: Decimal,
    pub open_24h: Decimal,
    pub volume_24h: Decimal,
    pub low_24h: Decimal,
    pub high_24h: Decimal,
    pub volume_30d: Decimal,
    pub best_bid: Decimal,
    pub best_bid_size: Decimal,
    pub best_ask: Decimal,
    pub best_ask_size: Decimal,
    pub side: TickerSide,
    pub time: String,
    pub trade_id: u64,
    pub last_size: Decimal,
}
