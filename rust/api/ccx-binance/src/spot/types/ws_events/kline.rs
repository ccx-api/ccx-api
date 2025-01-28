use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::PascalString;
use smart_string::SmartString;

use crate::spot::types::timestamp::BinanceTimestamp;

// {
//   "e": "kline",         // Event type
//   "E": 1672515782136,   // Event time
//   "s": "BNBBTC",        // Symbol
//   "k": {
//     "t": 1672515780000, // Kline start time
//     "T": 1672515839999, // Kline close time
//     "s": "BNBBTC",      // Symbol
//     "i": "1m",          // Interval
//     "f": 100,           // First trade ID
//     "L": 200,           // Last trade ID
//     "o": "0.0010",      // Open price
//     "c": "0.0020",      // Close price
//     "h": "0.0025",      // High price
//     "l": "0.0015",      // Low price
//     "v": "1000",        // Base asset volume
//     "n": 100,           // Number of trades
//     "x": false,         // Is this kline closed?
//     "q": "1.0000",      // Quote asset volume
//     "V": "500",         // Taker buy base asset volume
//     "Q": "0.500",       // Taker buy quote asset volume
//     "B": "123456"       // Ignore
//   }
// }

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Kline {
    /// Event time.
    #[serde(rename = "E")]
    pub event_time: BinanceTimestamp,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: SmartString,
    /// Data.
    #[serde(rename = "k")]
    pub data: KlineData,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct KlineData {
    /// Kline start time.
    #[serde(rename = "t")]
    pub start_time: BinanceTimestamp,
    /// Kline close time.
    #[serde(rename = "T")]
    pub close_time: BinanceTimestamp,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: SmartString,
    /// Interval.
    #[serde(rename = "i")]
    pub interval: KlineInterval,
    /// First trade ID.
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    /// Last trade ID.
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    /// Open price.
    #[serde(rename = "o")]
    pub open_price: Decimal,
    /// Close price.
    #[serde(rename = "c")]
    pub close_price: Decimal,
    /// High price.
    #[serde(rename = "h")]
    pub high_price: Decimal,
    /// Low price.
    #[serde(rename = "l")]
    pub low_price: Decimal,
    /// Base asset volume.
    #[serde(rename = "v")]
    pub base_asset_volume: Decimal,
    /// Number of trades.
    #[serde(rename = "n")]
    pub number_of_trades: i64,
    /// Is this kline closed?
    #[serde(rename = "x")]
    pub is_closed: bool,
    /// Quote asset volume.
    #[serde(rename = "q")]
    pub quote_asset_volume: Decimal,
    /// Taker buy base asset volume.
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: Decimal,
    /// Taker buy quote asset volume.
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: Decimal,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumString, strum::IntoStaticStr,
)]
pub enum KlineInterval {
    #[strum(serialize = "1s")]
    Seconds1,
    #[strum(serialize = "1m")]
    Minutes1,
    #[strum(serialize = "3m")]
    Minutes3,
    #[strum(serialize = "5m")]
    Minutes5,
    #[strum(serialize = "15m")]
    Minutes15,
    #[strum(serialize = "30m")]
    Minutes30,
    #[strum(serialize = "1h")]
    Hours1,
    #[strum(serialize = "2h")]
    Hours2,
    #[strum(serialize = "4h")]
    Hours4,
    #[strum(serialize = "6h")]
    Hours6,
    #[strum(serialize = "8h")]
    Hours8,
    #[strum(serialize = "12h")]
    Hours12,
    #[strum(serialize = "1d")]
    Days1,
    #[strum(serialize = "3d")]
    Days3,
    #[strum(serialize = "1w")]
    Weeks1,
    #[strum(serialize = "1M")]
    Months1,
}

impl Serialize for KlineInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for KlineInterval {
    fn deserialize<D>(deserializer: D) -> Result<KlineInterval, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = PascalString::<3>::deserialize(deserializer)?;
        KlineInterval::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn it_deserializes_doc_example() {
        let json = r#"{
            "e": "kline",
            "E": 1672515782136,
            "s": "BNBBTC",
            "k": {
                "t": 1672515780000,
                "T": 1672515839999,
                "s": "BNBBTC",
                "i": "1m",
                "f": 100,
                "L": 200,
                "o": "0.0010",
                "c": "0.0020",
                "h": "0.0025",
                "l": "0.0015",
                "v": "1000",
                "n": 100,
                "x": false,
                "q": "1.0000",
                "V": "500",
                "Q": "0.500",
                "B": "123456"
            }
        }"#;
        // event_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
        let expected = Kline {
            event_time: BinanceTimestamp::from_epoch_millis(1672515782136).unwrap(),
            symbol: "BNBBTC".into(),
            data: KlineData {
                start_time: BinanceTimestamp::from_epoch_millis(1672515780000).unwrap(),
                close_time: BinanceTimestamp::from_epoch_millis(1672515839999).unwrap(),
                symbol: "BNBBTC".into(),
                interval: KlineInterval::Minutes1,
                first_trade_id: 100,
                last_trade_id: 200,
                open_price: dec!(0.0010),
                close_price: dec!(0.0020),
                high_price: dec!(0.0025),
                low_price: dec!(0.0015),
                base_asset_volume: dec!(1000),
                number_of_trades: 100,
                is_closed: false,
                quote_asset_volume: dec!(1.0000),
                taker_buy_base_asset_volume: dec!(500),
                taker_buy_quote_asset_volume: dec!(0.500),
            },
        };
        let actual = serde_json::from_str::<Kline>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
