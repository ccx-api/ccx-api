use serde::Deserialize;

use crate::Decimal;

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBook {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub volume: Decimal,
}
