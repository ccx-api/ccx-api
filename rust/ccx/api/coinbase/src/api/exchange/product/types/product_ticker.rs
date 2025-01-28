use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductTicker {
    /// The best ask price in the quote currency.
    pub ask: Decimal,
    /// The best bid price in the quote currency.
    pub bid: Decimal,
    /// The total trading volume for the last 24 hours in the base currency.
    pub volume: Decimal,
    /// The unique identifier for the trade.
    pub trade_id: i64,
    /// The price of the trade in the quote currency.
    pub price: Decimal,
    /// The size of the trade in the base currency.
    pub size: Decimal,
    /// The time of the trade.
    pub time: DtCoinbasePrime,
}
