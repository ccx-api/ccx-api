use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct AggTrade {
    // TODO: I saw only null values for the id so far...
    /// Aggregate tradeId.
    #[serde(rename = "a")]
    pub id: Option<u64>,
    /// First tradeId.
    #[serde(rename = "f")]
    pub first_trade_id: Option<u64>,
    /// Last tradeId.
    #[serde(rename = "l")]
    pub last_trade_id: Option<u64>,
    /// Price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// Quantity.
    #[serde(rename = "q")]
    pub qty: Decimal,
    /// Timestamp.
    #[serde(rename = "T")]
    pub time: u64,
    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    /// Was the trade the best price match?
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn it_deserializes_doc_example() {
        let json = r#"{
            "a": null,
            "f": null,
            "l": null,
            "p": "46782.67",
            "q": "0.0038",
            "T": 1641380483000,
            "m": false,
            "M": true
        }"#;
        let expected = AggTrade {
            id: None,
            first_trade_id: None,
            last_trade_id: None,
            price: dec!(46782.67),
            qty: dec!(0.0038),
            time: 1641380483000,
            is_buyer_maker: false,
            is_best_match: true,

        };
        let actual = serde_json::from_str::<AggTrade>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
