use rust_decimal::Decimal;
use smart_string::SmartString;

use crate::spot::types::timestamp::BinanceTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct AggTrade {
    /// Event time.
    #[serde(rename = "E")]
    pub event_time: BinanceTimestamp,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: SmartString,
    /// Aggregate trade ID.
    #[serde(rename = "a")]
    pub aggregate_trade_id: i64,
    /// Price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// Quantity.
    #[serde(rename = "q")]
    pub quantity: Decimal,
    /// First trade ID.
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    /// Last trade ID.
    #[serde(rename = "l")]
    pub last_trade_id: i64,
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
            "e": "aggTrade",
            "E": 1672515782136,
            "s": "BNBBTC",
            "a": 12345,
            "p": "0.001",
            "q": "100",
            "f": 100,
            "l": 105,
            "T": 1672515782136,
            "m": true,
            "M": true
        }"#;
        let expected = AggTrade {
            event_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
            symbol: "BNBBTC".into(),
            aggregate_trade_id: 12345,
            price: dec!(0.001),
            quantity: dec!(100),
            first_trade_id: 100,
            last_trade_id: 105,
            trade_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
            is_buyer_market_maker: true,
        };
        let actual = serde_json::from_str::<AggTrade>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
