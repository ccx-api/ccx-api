use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductTicker {
    /// The unique identifier for the trade.
    pub trade_id: i64,
    /// The price of the trade in the quote currency.
    pub price: String,
    /// The size of the trade in the base currency.
    pub size: String,
    /// The time of the trade.
    pub time: String,
    /// The best bid price in the quote currency.
    pub bid: String,
    /// The best ask price in the quote currency.
    pub ask: String,
    /// The total trading volume for the last 24 hours in the base currency.
    pub volume: String,
}
