use ccx_api_lib::Decimal;
use serde::Deserialize;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Deserialize, Clone)]
pub struct AuctionMessage {
    pub sequence: u64,
    pub product_id: Atom,
    pub auction_state: String,
    pub best_bid_price: Decimal,
    pub best_bid_size: Decimal,
    pub best_ask_price: Decimal,
    pub best_ask_size: Decimal,
    pub open_price: Decimal,
    pub open_size: Decimal,
    pub can_open: String,
    pub timestamp: String,
}
