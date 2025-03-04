use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_with::TimestampMilliSeconds;
use serde_with::formats::Flexible;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use smallvec::SmallVec;
use smart_string::SmartString;

use crate::proto::{PublicRequest, Request, Response};

/// Retrieve order book
///
/// Order book will be sorted by price from high to low on bids; low to high on asks
///
/// ## Parameters
/// See [SpotOrderBookRequest]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, Builder)]
#[cfg_attr(test, derive(PartialEq))]
pub struct OrderBook {
    currency_pair: SmartString,
    /// Order depth. 0 means no aggregation is applied. default to 0
    #[serde(rename = "interval")]
    order_depth: Option<Decimal>,
    limit: Option<u32>,
    with_id: Option<bool>,
}

impl OrderBook {
    pub fn currency_pair(currency_pair: impl Into<SmartString>) -> Self {
        Self::builder().currency_pair(currency_pair.into()).build()
    }
}

impl Request for OrderBook {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/order_book";

    type Response = OrderBookResponse;
}

impl PublicRequest for OrderBook {}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct OrderBookResponse {
    /// Order book ID, which is updated whenever the order book is changed.
    ///
    /// Valid only when with_id is set to true
    pub id: Option<SmartString>,
    /// The timestamp of the response data being generated (in milliseconds)
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub current: DateTime<Utc>,
    /// The timestamp of when the orderbook last changed (in milliseconds)
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update: DateTime<Utc>,
    /// Ask orders
    pub asks: SmallVec<[PriceAndAmount; 1]>,
    /// Bid orders
    pub bids: SmallVec<[PriceAndAmount; 1]>,
}

impl Response for OrderBookResponse {}

/// Order price and amount (volume)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PriceAndAmount {
    pub price: Decimal,
    pub amount: Decimal,
}

impl<'de> Deserialize<'de> for PriceAndAmount {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let [price, amount] = <[Decimal; 2]>::deserialize(deserializer)?;
        Ok(Self { price, amount })
    }
}

impl From<(Decimal, Decimal)> for PriceAndAmount {
    fn from((price, amount): (Decimal, Decimal)) -> Self {
        Self { price, amount }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use smallvec::smallvec;

    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{
  "current": 1623898993123,
  "update": 1623898993121,
  "asks": [
    [
      "1.52",
      "1.151"
    ],
    [
      "1.53",
      "1.218"
    ]
  ],
  "bids": [
    [
      "1.17",
      "201.863"
    ],
    [
      "1.16",
      "725.464"
    ]
  ]
}"#;
        let res: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            OrderBookResponse {
                id: None,
                current: DateTime::from_timestamp_millis(1623898993123).unwrap(),
                update: DateTime::from_timestamp_millis(1623898993121).unwrap(),
                asks: smallvec![
                    PriceAndAmount {
                        price: dec!(1.52),
                        amount: dec!(1.151)
                    },
                    PriceAndAmount {
                        price: dec!(1.53),
                        amount: dec!(1.218)
                    }
                ],
                bids: smallvec![
                    PriceAndAmount {
                        price: dec!(1.17),
                        amount: dec!(201.863),
                    },
                    PriceAndAmount {
                        price: dec!(1.16),
                        amount: dec!(725.464),
                    }
                ],
            }
        );
    }
}
