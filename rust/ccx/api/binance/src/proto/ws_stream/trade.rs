use serde::{Deserialize, Serialize};

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TradeEvent {
    /// Event type.
    #[serde(skip, rename = "e")]
    pub event_type: (),
    /// Event time.
    #[serde(rename = "E")]
    pub event_time: u64,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: Atom,
    /// Trade ID.
    #[serde(rename = "t")]
    pub id: u64,
    /// Price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// Quantity.
    #[serde(rename = "q")]
    pub qty: Decimal,
    /// Buyer order ID.
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    /// Seller order ID.
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    /// Trade time.
    #[serde(rename = "T")]
    pub time: u64,
    /// Is the buyer the market maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    /// Ignore.
    #[serde(rename = "M")]
    pub is_best_match: bool,
}
