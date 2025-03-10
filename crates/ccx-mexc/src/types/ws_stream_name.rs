use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use smart_string::DisplayExt;
use smart_string::SmartString;

use crate::types::ws_events::KlineInterval;

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
    /// Partial Book Depth Streams
    /// This stream pushes limited level depth information. The “levels” indicate the number of order levels for buy and sell orders, which can be 5, 10, or 20 levels.
    BookDepth { symbol: SmartString, level: u16 },
}

#[derive(Debug, Clone, Eq, PartialEq, strum::EnumString, strum::IntoStaticStr)]
pub enum KlineTimezone {
    #[strum(serialize = "+08:00")]
    Asia,
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
            StreamName::BookDepth { symbol, level } => {
                write!(f, "spot@public.limit.depth.v3.api.pb@{symbol}@{level}")?;
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
        let name: SmartString<254> = self.to_fmt();
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
                    let level = e.trim_start_matches("depth");
                    let level = level.parse().map_err(serde::de::Error::custom)?;

                    Ok(StreamName::BookDepth { symbol, level })
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
                interval: KlineInterval::Min1,
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
                level: 10,
            },
            StreamName::BookDepth {
                symbol: "bnbbtc".into(),
                level: 5,
            },
        ];
        for case in cases {
            let serialized = serde_json::to_string(&case).expect("serialization failed");
            let deserialized =
                serde_json::from_str::<StreamName>(&serialized).expect("deserialization failed");
            assert_eq!(case, deserialized);
        }
    }
}
