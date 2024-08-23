use serde::Deserialize;

use super::auction::AuctionMessage;
use super::subscribe::SubscribeResponse;
use super::ticker::TickerMessage;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientMessage {
    Ticker(TickerMessage),
    Auction(AuctionMessage),
    Subscriptions(SubscribeResponse),
    Error(ErrorMessage),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorMessage {
    pub message: String,
}
