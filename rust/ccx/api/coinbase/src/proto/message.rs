use serde::Deserialize;

use super::auction::AuctionMessage;
use super::ticker::TickerMessage;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientMessage {
    Ticker(TickerMessage),
    Auction(AuctionMessage),
    Error(ErrorMessage),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorMessage {
    pub message: String,
}
