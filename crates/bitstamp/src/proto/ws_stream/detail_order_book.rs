use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DetailOrderBookEvent {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<DetailOrderBookEntry>,
    pub asks: Vec<DetailOrderBookEntry>,
}

/// Minimalistic representation of event occurred in Order Book. Received from
/// [Live detail order book] subscription.
///
/// ## Deserialization:
/// Deserialized via `(Decimal, Decimal, String)` because Bitstamp sends each
/// event as a list of exactly three strings with decimals encoded in first two.
/// First value represents `price`, second value represents `amount` and third
/// value represents `order_id`.
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "(Decimal, Decimal, String)")]
pub struct DetailOrderBookEntry {
    /// Unique order identifier.
    pub order_id: String,
    /// Price ordered.
    pub price: Decimal,
    /// Amount ordered.
    pub amount: Decimal,
}

impl From<(Decimal, Decimal, String)> for DetailOrderBookEntry {
    fn from(value: (Decimal, Decimal, String)) -> Self {
        Self {
            order_id: value.2,
            price: value.0,
            amount: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ws_stream::Event;

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "data":{
                "timestamp":"1692095753",
                "microtimestamp":"1692095753831370",
                "bids":[
                    ["29376","1.42905047", "1"],
                    ["29375","0.70644900", "2"],
                    ["29373","0.20000000", "3"],
                    ["29372","0.52500000", "4"],
                    ["29371","0.15000000", "5"]
                ],
                "asks":[
                    ["29377","0.09638168", "6"],
                    ["29378","0.00222701", "7"],
                    ["29379","0.09643274", "8"],
                    ["29380","0.05000000", "9"],
                    ["29381","2.98042494", "10"]
                ]
            },
            "channel":"detail_order_book_btcusd",
            "event":"data"
        }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(
            res.is_ok(),
            "Failed to deserialize detail_order_book event: {:?}",
            res
        );
    }
}
