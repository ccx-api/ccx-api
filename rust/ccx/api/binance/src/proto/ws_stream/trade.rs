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

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_decode_doc() {
        let json = r#"{
          "e": "trade",
          "E": 1672515782136,
          "s": "BNBBTC",
          "t": 12345,
          "p": "0.001",
          "q": "100",
          "T": 1672515782136,
          "m": true,
          "M": true
        }"#;
        let expected = TradeEvent {
            event_type: (),
            event_time: 1672515782136,
            symbol: Atom::from("BNBBTC"),
            id: 12345,
            price: dec!(0.001),
            qty: dec!(100),
            time: 1672515782136,
            is_buyer_maker: true,
            is_best_match: true,
        };
        let event: TradeEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event, expected);
    }

    #[test]
    fn test_decode_live_1() {
        let json = r#"{"e":"trade","E":1722723254022,"s":"BTCUSDT","t":3717726327,"p":"60668.01000000","q":"0.00009000","T":1722723254021,"m":true,"M":true}"#;
        let expected = TradeEvent {
            event_type: (),
            event_time: 1722723254022,
            symbol: Atom::from("BTCUSDT"),
            id: 3717726327,
            price: dec!(60668.01),
            qty: dec!(0.00009),
            time: 1722723254021,
            is_buyer_maker: true,
            is_best_match: true,
        };
        let event: TradeEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event, expected);
    }
}
