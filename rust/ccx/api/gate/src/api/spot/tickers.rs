use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;
use crate::util::dt_gate::DtGate;
use crate::util::maybe_str;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotTickersRequest {
    pub currency_pair: Option<SmartString>,
    pub timezone: Option<Timezone>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Timezone {
    Utc0,
    Utc8,
    All,
}

impl Request for SpotTickersRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    const PATH: &'static str = "spot/tickers";
    const IS_PUBLIC: bool = true;
    type Response = SmallVec<[SpotTicker; 1]>;
}

// » currency_pair 	string 	Currency pair
// » last 	string 	Last trading price
// » lowest_ask 	string 	Recent lowest ask
// » highest_bid 	string 	Recent highest bid
// » change_percentage 	string 	Change percentage in the last 24h
// » change_utc0 	string 	utc0 timezone, the percentage change in the last 24 hours
// » change_utc8 	string 	utc8 timezone, the percentage change in the last 24 hours
// » base_volume 	string 	Base currency trade volume in the last 24h
// » quote_volume 	string 	Quote currency trade volume in the last 24h
// » high_24h 	string 	Highest price in 24h
// » low_24h 	string 	Lowest price in 24h
// » etf_net_value 	string 	ETF net value
// » etf_pre_net_value 	string|null 	ETF previous net value at re-balancing time
// » etf_pre_timestamp 	integer(int64)|null 	ETF previous re-balancing time
// » etf_leverage 	string|null 	ETF current leverage

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotTicker {
    /// Currency pair
    pub currency_pair: SmartString,
    /// Last trading price
    #[serde(with = "maybe_str")]
    pub last: Option<Decimal>,
    /// Recent lowest ask
    #[serde(with = "maybe_str")]
    pub lowest_ask: Option<Decimal>,
    /// Recent highest bid
    #[serde(with = "maybe_str")]
    pub highest_bid: Option<Decimal>,
    /// Change percentage in the last 24h
    #[serde(with = "maybe_str")]
    pub change_percentage: Option<Decimal>,
    /// utc0 timezone, the percentage change in the last 24 hours
    #[serde(with = "maybe_str", default)]
    pub change_utc0: Option<Decimal>,
    /// utc8 timezone, the percentage change in the last 24 hours
    #[serde(with = "maybe_str", default)]
    pub change_utc8: Option<Decimal>,
    /// Base currency trade volume in the last 24h
    #[serde(with = "maybe_str")]
    pub base_volume: Option<Decimal>,
    /// Quote currency trade volume in the last 24h
    #[serde(with = "maybe_str")]
    pub quote_volume: Option<Decimal>,
    /// Highest price in 24h
    #[serde(with = "maybe_str")]
    pub high_24h: Option<Decimal>,
    /// Lowest price in 24h
    #[serde(with = "maybe_str")]
    pub low_24h: Option<Decimal>,
    /// ETF net value
    #[serde(with = "maybe_str", default)]
    pub etf_net_value: Option<Decimal>,
    /// ETF previous net value at re-balancing time
    #[serde(with = "maybe_str", default)]
    pub etf_pre_net_value: Option<Decimal>,
    /// ETF previous re-balancing time
    #[serde(with = "maybe_str", default)]
    pub etf_pre_timestamp: Option<DtGate>,
    /// ETF current leverage
    #[serde(with = "maybe_str", default)]
    pub etf_leverage: Option<Decimal>,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;
    use crate::GateApi;

    impl<S: GateSigner> GateApi<S> {
        /// # Retrieve ticker information
        ///
        /// Return only related data if currency_pair is specified; otherwise return all of them.
        ///
        /// ## Parameters
        ///
        /// * `currency_pair` - Currency pair
        /// * `timezone` - Timezone
        pub async fn spot_tickers(
            &self,
            currency_pair: Option<SmartString>,
            timezone: Option<Timezone>,
        ) -> Result<<SpotTickersRequest as Request>::Response, RequestError> {
            self.request(&SpotTickersRequest {
                currency_pair,
                timezone,
            })
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_deserialize() {
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
        let res: SpotTicker = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            SpotTicker {
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
                etf_pre_timestamp: Some(DtGate::from_timestamp(1611244800000)),
                etf_leverage: Some(dec!(2.2803019447281203)),
            }
        );
    }
}
