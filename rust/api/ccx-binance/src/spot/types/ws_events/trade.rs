use rust_decimal::Decimal;
use smart_string::SmartString;

use crate::spot::types::timestamp::BinanceTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct TradeEvent {
    /// Event time.
    #[serde(rename = "E")]
    pub event_time: BinanceTimestamp,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: SmartString,
    /// Trade ID.
    #[serde(rename = "t")]
    pub trade_id: i64,
    /// Price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// Quantity.
    #[serde(rename = "q")]
    pub quantity: Decimal,
    /// Trade time.
    #[serde(rename = "T")]
    pub trade_time: BinanceTimestamp,
    /// Is the buyer the market maker?
    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn it_deserializes_doc_example() {
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
            event_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
            symbol: SmartString::from("BNBBTC"),
            trade_id: 12345,
            price: dec!(0.001),
            quantity: dec!(100),
            trade_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
            is_buyer_market_maker: true,
        };
        let actual = serde_json::from_str::<TradeEvent>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
