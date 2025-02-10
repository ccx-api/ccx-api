use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use smart_string::DisplayExt;
use smart_string::SmartString;

use crate::spot::types::ws_events::KlineInterval;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamName {
    /// Aggregate Trade Streams.
    AggTrade { symbol: SmartString },
    /// Trade Streams.
    Trade { symbol: SmartString },
    /// Kline/Candlestick Streams.
    Kline {
        symbol: SmartString,
        interval: KlineInterval,
        timezone: Option<KlineTimezone>,
    },
    /// Individual Symbol Mini Ticker Stream.
    ///
    /// 24hr rolling window mini-ticker statistics.
    /// These are NOT the statistics of the UTC day,
    /// but a 24hr rolling window for the previous 24hrs.
    MiniTicker { symbol: SmartString },
    /// Individual symbol ticker.
    Ticker { symbol: SmartString },
    /// Individual Symbol Rolling Window Statistics Streams.
    ///
    /// Rolling window ticker statistics for a single symbol, computed over multiple windows.
    ///
    /// Note: This stream is different from the <symbol>@ticker stream.
    /// The open time "O" always starts on a minute, while the closing time "C" is the current time
    /// of the update. As such, the effective window might be up to 59999ms wider than
    /// <window_size>.
    WindowTicker {
        symbol: SmartString,
        window_size: TickerWindow,
    },
    /// Individual Symbol Book Ticker Streams.
    BookTicker { symbol: SmartString },
    /// Average Price.
    AvgPrice { symbol: SmartString },
    /// Diff. Book Depth Streams.
    ///
    /// Order book price and quantity depth updates used to locally manage an order book.
    BookDepth {
        symbol: SmartString,
        /// Partial Book Depth Streams.
        ///
        /// Top levels of the order book for a symbol.
        /// Valid <levels> are 5, 10, or 20.
        ///
        /// [See](https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#partial-book-depth-streams)
        levels: Option<u16>,
        update_speed: DepthUpdateSpeed,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, strum::EnumString, strum::IntoStaticStr)]
pub enum KlineTimezone {
    #[strum(serialize = "+08:00")]
    Asia,
}

#[derive(Debug, Clone, Eq, PartialEq, strum::EnumString, strum::IntoStaticStr)]
pub enum DepthUpdateSpeed {
    #[strum(serialize = "100ms")]
    Ms100,
    #[strum(serialize = "1000ms")]
    Ms1000,
}

#[derive(Debug, Clone, Eq, PartialEq, strum::EnumString, strum::IntoStaticStr)]
pub enum TickerWindow {
    #[strum(serialize = "1h")]
    Hours1,
    #[strum(serialize = "4h")]
    Hours4,
    #[strum(serialize = "1d")]
    Days1,
}

impl fmt::Display for StreamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamName::AggTrade { symbol } => {
                f.write_str(symbol)?;
                f.write_str("@aggTrade")?;
            }
            StreamName::Trade { symbol } => {
                f.write_str(symbol)?;
                f.write_str("@trade")?;
            }
            StreamName::Kline {
                symbol,
                interval,
                timezone,
            } => {
                f.write_str(symbol)?;
                f.write_str("@kline_")?;
                f.write_str(interval.into())?;
                if let Some(timezone) = timezone {
                    f.write_str("@")?;
                    f.write_str(timezone.into())?;
                }
            }
            StreamName::MiniTicker { symbol } => {
                f.write_str(symbol)?;
                f.write_str("@miniTicker")?;
            }
            StreamName::Ticker { symbol } => {
                f.write_str(symbol)?;
                f.write_str("@ticker")?;
            }
            StreamName::WindowTicker {
                symbol,
                window_size,
            } => {
                f.write_str(symbol)?;
                f.write_str("@ticker_")?;
                f.write_str(window_size.into())?;
            }
            StreamName::BookTicker { symbol } => {
                f.write_str(symbol)?;
                f.write_str("@bookTicker")?;
            }
            StreamName::AvgPrice { symbol } => {
                f.write_str(symbol)?;
                f.write_str("@avgPrice")?;
            }
            StreamName::BookDepth {
                symbol,
                levels,
                update_speed,
            } => {
                f.write_str(symbol)?;
                f.write_str("@depth")?;
                if let Some(levels) = levels {
                    use std::fmt::Write;
                    write!(f, "{}", levels).map_err(serde::ser::Error::custom)?;
                }
                match update_speed {
                    DepthUpdateSpeed::Ms100 => f.write_str("@100ms")?,
                    DepthUpdateSpeed::Ms1000 => {}
                }
            }
        }
        Ok(())
    }
}

impl Serialize for StreamName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut name: SmartString<254> = self.to_fmt();
        name.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StreamName {
    fn deserialize<D>(deserializer: D) -> Result<StreamName, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = SmartString::<254>::deserialize(deserializer)?;

        let mut parts = name.split('@');

        let symbol = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing symbol"))?;
        let symbol = SmartString::from(symbol);

        let event = parts.next();

        let third_part = parts.next();

        match event {
            None => Err(serde::de::Error::custom("unknown event type")),
            Some(event) => match event {
                "aggTrade" => Ok(StreamName::AggTrade { symbol }),
                "trade" => Ok(StreamName::Trade { symbol }),
                e if e.starts_with("kline_") => {
                    let interval = KlineInterval::from_str(e.trim_start_matches("kline_"))
                        .map_err(|e| {
                            format_args!("unknown interval {e}").to_fmt::<SmartString<62>>()
                        })
                        .map_err(serde::de::Error::custom)?;
                    let timezone = third_part
                        .map(|s| {
                            s.parse()
                                .map_err(|e| {
                                    format_args!("unknown timezone {e}").to_fmt::<SmartString<62>>()
                                })
                                .map_err(serde::de::Error::custom)
                        })
                        .transpose()?;
                    Ok(StreamName::Kline {
                        symbol,
                        interval,
                        timezone,
                    })
                }
                "miniTicker" => Ok(StreamName::MiniTicker { symbol }),
                "ticker" => Ok(StreamName::Ticker { symbol }),
                e if e.starts_with("ticker_") => {
                    let window_size = e
                        .trim_start_matches("ticker_")
                        .parse()
                        .map_err(|e| {
                            format_args!("unknown window size {e}").to_fmt::<SmartString<62>>()
                        })
                        .map_err(serde::de::Error::custom)?;
                    Ok(StreamName::WindowTicker {
                        symbol,
                        window_size,
                    })
                }
                "bookTicker" => Ok(StreamName::BookTicker { symbol }),
                "avgPrice" => Ok(StreamName::AvgPrice { symbol }),
                e if e.starts_with("depth") => {
                    let levels = e.trim_start_matches("depth");
                    let levels = if !levels.is_empty() {
                        Some(levels.parse().map_err(serde::de::Error::custom)?)
                    } else {
                        None
                    };
                    let update_speed = third_part
                        .map_or(Ok(DepthUpdateSpeed::Ms1000), |s| {
                            s.parse().map_err(|e| {
                                format_args!("unknown update speed {e}").to_fmt::<SmartString<62>>()
                            })
                        })
                        .map_err(serde::de::Error::custom)?;
                    Ok(StreamName::BookDepth {
                        symbol,
                        levels,
                        update_speed,
                    })
                }

                _ => Err(serde::de::Error::custom("unknown event type")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_and_deserializes() {
        let cases = [
            StreamName::AggTrade {
                symbol: "bnbbtc".into(),
            },
            StreamName::Trade {
                symbol: "bnbbtc".into(),
            },
            StreamName::Kline {
                symbol: "bnbbtc".into(),
                interval: KlineInterval::Minutes1,
                timezone: None,
            },
            StreamName::MiniTicker {
                symbol: "bnbbtc".into(),
            },
            StreamName::Ticker {
                symbol: "bnbbtc".into(),
            },
            StreamName::WindowTicker {
                symbol: "bnbbtc".into(),
                window_size: TickerWindow::Hours1,
            },
            StreamName::BookTicker {
                symbol: "bnbbtc".into(),
            },
            StreamName::AvgPrice {
                symbol: "bnbbtc".into(),
            },
            StreamName::BookDepth {
                symbol: "bnbbtc".into(),
                levels: Some(5),
                update_speed: DepthUpdateSpeed::Ms100,
            },
            StreamName::BookDepth {
                symbol: "bnbbtc".into(),
                levels: None,
                update_speed: DepthUpdateSpeed::Ms1000,
            },
        ];
        for case in cases {
            println!("{:?}", case);
            let serialized = serde_json::to_string(&case).expect("serialization failed");
            let deserialized =
                serde_json::from_str::<StreamName>(&serialized).expect("deserialization failed");
            assert_eq!(case, deserialized);
        }
    }
}
