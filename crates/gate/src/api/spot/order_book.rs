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

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PublicRequest;
use crate::api::Request;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SpotOrderBookRequest {
    pub currency_pair: SmartString,
    /// Order depth. 0 means no aggregation is applied. default to 0
    #[serde(rename = "interval")]
    pub order_depth: Option<Decimal>,
    pub limit: Option<u32>,
    pub with_id: Option<bool>,
}

impl SpotOrderBookRequest {
    pub fn currency_pair(currency_pair: SmartString) -> Self {
        Self {
            currency_pair,
            order_depth: None,
            limit: None,
            with_id: None,
        }
    }
}

impl PublicRequest for SpotOrderBookRequest {}

impl Request for SpotOrderBookRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = SpotOrderBookResponse;
}

#[derive(Debug, Clone)]
pub struct OrderBook {}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SpotOrderBookResponse {
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

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::api::spot::SpotApi;
    use crate::client::rest::RequestError;

    impl<S> SpotApi<S> {
        /// Retrieve order book
        ///
        /// Order book will be sorted by price from high to low on bids; low to high on asks
        ///
        /// ## Parameters
        /// See [SpotOrderBookRequest]
        pub async fn order_book(
            &self,
            request: &SpotOrderBookRequest,
        ) -> Result<SpotOrderBookResponse, RequestError> {
            self.0.request("/spot/order_book", request).await
        }
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
        let res: SpotOrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            SpotOrderBookResponse {
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
