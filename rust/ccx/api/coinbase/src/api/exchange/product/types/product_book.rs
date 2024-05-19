use crate::api::exchange::prelude::*;
use crate::DtCoinbasePrime;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductBook {
    /// The best ask price in the quote currency.
    pub asks: Vec<ProductBookItem>,
    /// The best bid price in the quote currency.
    pub bids: Vec<ProductBookItem>,
    /// The sequence number of the last event.
    // pub sequence: f64,
    /// The current auction mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auction_mode: Option<bool>,
    /// The current auction state.
    #[serde(default)]
    pub auction: Option<ProductBookAuction>,
    /// The time of the last event.
    pub time: DtCoinbasePrime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductBookItem {
    /// The price of the order.
    pub price: Decimal,
    /// The size of the order.
    pub size: Decimal,
    /// The number of orders at this price.
    pub num_orders: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductBookAuction {
    /// The current auction price.
    pub open_price: Decimal,
    /// The current auction size.
    pub open_size: Decimal,
    /// Best bid price.
    pub best_bid_price: Decimal,
    /// Best bid size.
    pub best_bid_size: Decimal,
    /// Best ask price.
    pub best_ask_price: Decimal,
    /// Best ask size.
    pub best_ask_size: Decimal,
    /// The current auction state.
    pub auction_state: String,
    /// The current auction time.
    pub can_open: String,
    /// The current auction time.
    pub time: DtCoinbasePrime,
}

#[cfg(test)]
mod tests {
    use ccx_coinbase_examples_util::d;

    use super::*;

    #[test]
    fn test_deserialize_doc() {
        let json = r#"{
          "sequence": 13051505638,
          "bids": [
            [
              "6247.58",
              "6.3578146",
              2
            ]
          ],
          "asks": [
            [
              "6251.52",
              "2",
              1
            ]
          ],
          "time": "2021-02-12T01:09:23.334723Z"
        }"#;
        let sample = ProductBook {
            asks: vec![ProductBookItem {
                price: d("6251.52"),
                size: d("2"),
                num_orders: 1,
            }],
            bids: vec![ProductBookItem {
                price: d("6247.58"),
                size: d("6.3578146"),
                num_orders: 2,
            }],
            // sequence: 13051505638.0,
            auction_mode: None,
            auction: None,
            time: DtCoinbasePrime::parse_from_str("2021-02-12T01:09:23.334723Z").unwrap(),
        };
        let order: ProductBook = serde_json::from_str(json).unwrap();
        assert_eq!(order, sample);
    }
}
