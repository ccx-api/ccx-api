use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct OrderBookEvent {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

/// Minimalistic representation of event occurred in Order Book. Received from
/// [Live order book] and [Live full order book] subscriptions. If you also need
/// order id, please check [`DetailOrderBookEvent`].
///
/// ## Deserialization:
/// Deserialized via `[Decimal; 2]` because Bitstamp sends each event as a
/// list of exactly two strings with decimals encoded in them. First string
/// represents `price` and the second one represents `amount`.
///
/// [`DetailOrderBookEvent`]: super::detail_order_book::DetailOrderBookEvent
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "[Decimal; 2]")]
pub struct OrderBookEntry {
    /// Price ordered.
    pub price: Decimal,
    /// Amount ordered.
    pub amount: Decimal,
}

impl From<[Decimal; 2]> for OrderBookEntry {
    fn from(value: [Decimal; 2]) -> Self {
        Self {
            price: value[0],
            amount: value[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ws_stream::Event;

    #[test]
    fn test_deserialize_order_book() {
        let json = r#"{
            "data":{
                "timestamp":"1692095753",
                "microtimestamp":"1692095753831370",
                "bids":[
                    ["29376","1.42905047"],
                    ["29375","0.70644900"],
                    ["29373","0.20000000"],
                    ["29372","0.52500000"],
                    ["29371","0.15000000"]
                ],
                "asks":[
                    ["29377","0.09638168"],
                    ["29378","0.00222701"],
                    ["29379","0.09643274"],
                    ["29380","0.05000000"],
                    ["29381","2.98042494"]
                ]
            },
            "channel":"order_book_btcusd",
            "event":"data"
        }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(
            res.is_ok(),
            "Failed to deserialize order_book_event: {:?}",
            res
        );
    }

    #[test]
    fn test_deserialize_order_book_diff() {
        let json = r#"{
            "data":{
                "timestamp":"1692095753",
                "microtimestamp":"1692095753831370",
                "bids":[
                    ["29376","1.42905047"],
                    ["29375","0.70644900"],
                    ["29373","0.20000000"],
                    ["29372","0.52500000"],
                    ["29371","0.15000000"]
                ],
                "asks":[
                    ["29377","0.09638168"],
                    ["29378","0.00222701"],
                    ["29379","0.09643274"],
                    ["29380","0.05000000"],
                    ["29381","2.98042494"]
                ]
            },
            "channel":"diff_order_book_btcusd",
            "event":"data"
        }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(
            res.is_ok(),
            "Failed to deserialize order_book_diff_event: {:?}",
            res
        );
    }
}
