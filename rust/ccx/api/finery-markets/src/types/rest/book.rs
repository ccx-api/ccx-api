use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::Pair;
use crate::types::Price;
use crate::types::Size;

pub const API_BOOK: &str = "api/book";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BookRequest {
    pub instrument: Pair,
    pub tradable: bool,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct BookResponse {
    pub bids: Vec<BookItem>,
    pub asks: Vec<BookItem>,
}

impl BookResponse {
    pub fn current_market_bid(&self) -> Option<&BookItem> {
        if self.bids.is_empty() {
            return None;
        }
        Some(&self.bids[0])
    }

    pub fn current_market_ask(&self) -> Option<&BookItem> {
        if self.asks.is_empty() {
            return None;
        }
        Some(&self.asks[0])
    }

    pub fn last_market_ask(&self) -> Option<&BookItem> {
        if self.asks.is_empty() {
            return None;
        }
        Some(&self.asks[self.asks.len() - 1])
    }
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct BookItem {
    /// 0
    /// Efx::Price
    /// Level price
    pub price: Price,
    /// 1
    /// Efx::Size
    /// Level total size
    pub total_size: Size,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_book() {
        let json = r#"
        {
            "instrument": "BTC-USD",
            "tradable": true
        }
        "#;
        test_serde_value_type::<BookRequest>(json);

        let json = r#"
        [
            [
                [
                    1100000000000,
                    11000000
                ],
                [
                    1099900000000,
                    100000000
                ]
            ],
            [
                [
                    1100100000000,
                    80000000
                ],
                [
                    1100200000000,
                    200000000
                ]
            ]
        ]
        "#;
        test_serde_response::<BookResponse>(json);
        test_serde_response_err::<BookResponse>();
    }
}
