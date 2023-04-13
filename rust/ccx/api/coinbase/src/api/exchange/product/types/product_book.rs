use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductBook {
    /// The best ask price in the quote currency.
    pub asks: Vec<ProductBookItem>,
    /// The best bid price in the quote currency.
    pub bids: Vec<ProductBookItem>,
    /// The sequence number of the last event.
    // pub sequence: f64,
    /// The current auction mode.
    pub auction_mode: bool,
    /// The current auction state.
    #[serde(default)]
    pub auction: Option<ProductBookAuction>,
    /// The time of the last event.
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductBookItem {
    /// The price of the order.
    pub price: Decimal,
    /// The size of the order.
    pub size: Decimal,
    /// The number of orders at this price.
    pub num_orders: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductBookAuction {
    /// The current auction price.
    pub open_price: Decimal,
    /// The current auction size.
    pub open_size: Decimal,
    /// Best bid price.
    pub best_bid_price: Decimal,
    /// Best bid size.
    pub best_bid_size: Decimal,
    /// Best ask price.
    pub best_ask_price: Decimal,
    /// Best ask size.
    pub best_ask_size: Decimal,
    /// The current auction state.
    pub auction_state: String,
    /// The current auction time.
    pub can_open: String,
    /// The current auction time.
    pub time: String,
}
