use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use smallvec::SmallVec;
use smart_string::SmartString;

use crate::proto::{PublicRequest, Request, Response};

/// # Retrieve ticker information
///
/// Return only related data if currency_pair is specified; otherwise return all of them.
///
/// ## Parameters
///
/// * `currency_pair` - Currency pair
/// * `timezone` - Timezone
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Default, Builder)]
#[builder(on(SmartString, into))]
pub struct SpotTickers {
    currency_pair: Option<SmartString>,
    timezone: Option<Timezone>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Timezone {
    Utc0,
    Utc8,
    All,
}

impl Request for SpotTickers {
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v4/spot/tickers";

    type Response = SmallVec<[SpotTickerResponse; 1]>;
}

impl PublicRequest for SpotTickers {}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SpotTickerResponse {
    /// Currency pair
    pub currency_pair: SmartString,
    /// Last trading price
    #[serde(default, with = "crate::util::maybe_str")]
    pub last: Option<Decimal>,
    /// Recent lowest ask
    #[serde(default, with = "crate::util::maybe_str")]
    pub lowest_ask: Option<Decimal>,
    /// Recent highest bid
    #[serde(default, with = "crate::util::maybe_str")]
    pub highest_bid: Option<Decimal>,
    /// Change percentage in the last 24h
    #[serde(default, with = "crate::util::maybe_str")]
    pub change_percentage: Option<Decimal>,
    /// utc0 timezone, the percentage change in the last 24 hours
    #[serde(default, with = "crate::util::maybe_str")]
    pub change_utc0: Option<Decimal>,
    /// utc8 timezone, the percentage change in the last 24 hours
    #[serde(default, with = "crate::util::maybe_str")]
    pub change_utc8: Option<Decimal>,
    /// Base currency trade volume in the last 24h
    #[serde(default, with = "crate::util::maybe_str")]
    pub base_volume: Option<Decimal>,
    /// Quote currency trade volume in the last 24h
    #[serde(default, with = "crate::util::maybe_str")]
    pub quote_volume: Option<Decimal>,
    /// Highest price in 24h
    #[serde(default, with = "crate::util::maybe_str")]
    pub high_24h: Option<Decimal>,
    /// Lowest price in 24h
    #[serde(default, with = "crate::util::maybe_str")]
    pub low_24h: Option<Decimal>,
    /// ETF net value
    #[serde(default, with = "crate::util::maybe_str")]
    #[serde()]
    pub etf_net_value: Option<Decimal>,
    /// ETF previous net value at re-balancing time
    #[serde(default, with = "crate::util::maybe_str")]
    pub etf_pre_net_value: Option<Decimal>,
    /// ETF previous re-balancing time
    #[serde_as(as = "Option<TimestampSeconds<i64, Flexible>>")]
    pub etf_pre_timestamp: Option<DateTime<Utc>>,
    /// ETF current leverage
    #[serde(default, with = "crate::util::maybe_str")]
    pub etf_leverage: Option<Decimal>,
}

impl Response for SpotTickerResponse {}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_deserialize_good() {
        let json = r#"{
    "currency_pair": "BTC3L_USDT",
    "last": "2.46140352",
    "lowest_ask": "2.477",
    "highest_bid": "2.4606821",
    "change_percentage": "-8.91",
    "change_utc0": "-8.91",
    "change_utc8": "-8.91",
    "base_volume": "656614.0845820589",
    "quote_volume": "1602221.66468375534639404191",
    "high_24h": "2.7431",
    "low_24h": "1.9863",
    "etf_net_value": "2.46316141",
    "etf_pre_net_value": "2.43201848",
    "etf_pre_timestamp": 1611244800,
    "etf_leverage": "2.2803019447281203"
}"#;
        let res: SpotTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            SpotTickerResponse {
                currency_pair: "BTC3L_USDT".into(),
                last: Some(dec!(2.46140352)),
                lowest_ask: Some(dec!(2.477)),
                highest_bid: Some(dec!(2.4606821)),
                change_percentage: Some(dec!(-8.91)),
                change_utc0: Some(dec!(-8.91)),
                change_utc8: Some(dec!(-8.91)),
                base_volume: Some(dec!(656614.0845820589)),
                quote_volume: Some(dec!(1602221.66468375534639404191)),
                high_24h: Some(dec!(2.7431)),
                low_24h: Some(dec!(1.9863)),
                etf_net_value: Some(dec!(2.46316141)),
                etf_pre_net_value: Some(dec!(2.43201848)),
                etf_pre_timestamp: DateTime::from_timestamp(1611244800, 0),
                etf_leverage: Some(dec!(2.2803019447281203)),
            }
        );
    }

    #[test]
    fn test_deserialize_empty_strings() {
        let json = r#"{
        "currency_pair": "USDG_USDT",
        "last": "1.9745",
        "lowest_ask": "",
        "highest_bid": "",
        "change_percentage": "0",
        "change_utc0": "0",
        "change_utc8": "0",
        "base_volume": "0",
        "quote_volume": "0",
        "high_24h": "0",
        "low_24h": "0"
    }"#;
        let res: SpotTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            SpotTickerResponse {
                currency_pair: "USDG_USDT".into(),
                last: Some(dec!(1.9745)),
                lowest_ask: None,
                highest_bid: None,
                change_percentage: Some(Decimal::ZERO),
                change_utc0: Some(Decimal::ZERO),
                change_utc8: Some(Decimal::ZERO),
                base_volume: Some(Decimal::ZERO),
                quote_volume: Some(Decimal::ZERO),
                high_24h: Some(Decimal::ZERO),
                low_24h: Some(Decimal::ZERO),
                etf_net_value: None,
                etf_pre_net_value: None,
                etf_pre_timestamp: None,
                etf_leverage: None,
            }
        );
    }
}
