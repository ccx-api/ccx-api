use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use serde_with::serde_as;
use smart_string::SmartString;

use crate::proto::{PublicRequest, Request, Response};

/// List all currency pairs supported by the API.
///
/// # Endpoint
/// `GET /spot/currency_pairs`
///
/// # Description
/// This endpoint retrieves a list of all currency pairs that are supported.
#[derive(Debug, Clone, Serialize)]
pub struct AllCurrencyPairs;

impl Request for AllCurrencyPairs {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/currency_pairs";

    type Response = Vec<CurrencyPairResponse>;
}

impl PublicRequest for AllCurrencyPairs {}

/// Get details of a specific currency pair.
///
/// # Endpoint
/// `GET /spot/currency_pairs/{currency_pair}`
///
/// # Description
/// This endpoint retrieves detailed information about a specific currency pair.
///
/// # Parameters
/// - `currency_pair`: The currency pair to retrieve details for.
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyPair {
    #[serde(skip)]
    currency_pair: String,
}

impl CurrencyPair {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            currency_pair: format!("{from}_{to}"),
        }
    }
}

impl Request for CurrencyPair {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/currency_pairs/{currency_pair}";

    type Response = CurrencyPairResponse;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let currency_pair = &self.currency_pair;

        format!("/api/v4/spot/currency_pairs/{currency_pair}").into()
    }
}

impl PublicRequest for CurrencyPair {}

/// Represents a spot currency pair.
#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CurrencyPairResponse {
    /// Currency pair identifier.
    #[serde(default, with = "crate::util::maybe_str")]
    pub id: Option<SmartString>,
    /// Base currency of the pair.
    #[serde(default, with = "crate::util::maybe_str")]
    pub base: Option<SmartString>,
    /// Quote currency of the pair.
    #[serde(default, with = "crate::util::maybe_str")]
    pub quote: Option<SmartString>,
    /// Trading fee associated with the currency pair.
    #[serde(default, with = "crate::util::maybe_str")]
    pub fee: Option<Decimal>,
    /// Minimum amount of base currency to trade, null means no limit.
    #[serde(default, with = "crate::util::maybe_str")]
    pub min_base_amount: Option<Decimal>,
    /// Minimum amount of quote currency to trade, null means no limit.
    #[serde(default, with = "crate::util::maybe_str")]
    pub min_quote_amount: Option<Decimal>,
    /// Maximum amount of base currency to trade, null means no limit.
    #[serde(default, with = "crate::util::maybe_str")]
    pub max_base_amount: Option<Decimal>,
    /// Maximum amount of quote currency to trade, null means no limit.
    #[serde(default, with = "crate::util::maybe_str")]
    pub max_quote_amount: Option<Decimal>,
    /// Amount scale precision.
    pub amount_precision: Option<u32>,
    /// Price scale precision.
    pub precision: Option<u32>,
    /// How the currency pair can be traded.
    ///
    /// More info in [TradeStatus]
    pub trade_status: Option<TradeStatus>,
    /// Sell start unix timestamp in seconds.
    #[serde_as(as = "Option<TimestampSeconds<i64, Flexible>>")]
    pub sell_start: Option<DateTime<Utc>>,
    /// Buy start unix timestamp in seconds.
    #[serde_as(as = "Option<TimestampSeconds<i64, Flexible>>")]
    pub buy_start: Option<DateTime<Utc>>,
}

impl Response for CurrencyPairResponse {}

/// How currency pair can be traded
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TradeStatus {
    /// Can be bought or sold
    Tradable,
    /// Cannot be bought or sold
    Untradable,
    /// Can be bought
    Buyable,
    /// Can be sold
    Sellable,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn deserialize_currency_pair() {
        let json = r#"{
            "id": "ETH_USDT",
            "base": "ETH",
            "quote": "USDT",
            "fee": "0.2",
            "min_base_amount": "0.001",
            "min_quote_amount": "1.0",
            "max_base_amount": "10000",
            "max_quote_amount": "10000000",
            "amount_precision": 3,
            "precision": 6,
            "trade_status": "tradable",
            "sell_start": 1516378650,
            "buy_start": 1516378650
        }"#;

        let expected = CurrencyPairResponse {
            id: Some("ETH_USDT".into()),
            base: Some("ETH".into()),
            quote: Some("USDT".into()),
            fee: Some(dec!(0.2)),
            min_base_amount: Some(dec!(0.001)),
            min_quote_amount: Some(dec!(1.0)),
            max_base_amount: Some(dec!(10000)),
            max_quote_amount: Some(dec!(10000000)),
            amount_precision: Some(3),
            precision: Some(6),
            trade_status: Some(TradeStatus::Tradable),
            sell_start: DateTime::from_timestamp(1516378650, 0),
            buy_start: DateTime::from_timestamp(1516378650, 0),
        };

        let actual: CurrencyPairResponse = serde_json::from_str(json).unwrap();
        assert_eq!(actual, expected);
    }
}
