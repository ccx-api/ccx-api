use smart_string::SmartString;

use crate::api::spot::OrderBookRow;
use crate::types::timestamp::BinanceTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct DepthUpdateEvent {
    /// Event time.
    #[serde(rename = "E")]
    pub event_time: BinanceTimestamp,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: SmartString,
    /// First update ID in event.
    #[serde(rename = "U")]
    pub first_update_id: i64,
    /// Last update ID in event.
    #[serde(rename = "u")]
    pub last_update_id: i64,
    /// Bids to be updated.
    #[serde(rename = "b")]
    pub bids: Vec<OrderBookRow>,
    /// Asks to be updated.
    #[serde(rename = "a")]
    pub asks: Vec<OrderBookRow>,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn it_deserializes_doc_example() {
        let json = r#"{
            "e": "depthUpdate",
            "E": 1672515782136,
            "s": "BNBBTC",
            "U": 157,
            "u": 160,
            "b": [
              [
                "0.0024",
                "10"
              ]
            ],
            "a": [
              [
                "0.0026",
                "100"
              ]
            ]
        }"#;
        let expected = DepthUpdateEvent {
            event_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
            symbol: SmartString::from("BNBBTC"),
            first_update_id: 157,
            last_update_id: 160,
            bids: vec![OrderBookRow {
                price: dec!(0.0024),
                qty: dec!(10),
            }],
            asks: vec![OrderBookRow {
                price: dec!(0.0026),
                qty: dec!(100),
            }],
        };
        let actual = serde_json::from_str::<DepthUpdateEvent>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
