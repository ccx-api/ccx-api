use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;

mod account_update;
mod agg_trade;
mod balance;
mod day_ticker;
mod kline;
mod mini_ticker;
mod order_trade;
mod orderbook_diff;
mod ticker;
mod trade;
mod ws_kline;

pub use account_update::*;
pub use agg_trade::*;
pub use balance::*;
pub use day_ticker::*;
pub use kline::*;
pub use mini_ticker::*;
pub use order_trade::*;
pub use orderbook_diff::*;
pub use ticker::*;
pub use trade::*;
pub use ws_kline::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WsEvent {
    AggTrade(AggTradeEvent),
    OrderBookDiff(OrderBookDiffEvent),
    Kline(KlineEvent),
    Trade(TradeEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(tag = "method", content = "params")]
pub enum WsCommand {
    #[serde(rename = "SUBSCRIBE")]
    Subscribe(Box<[WsSubscription]>),
    #[serde(rename = "SUBSCRIBE")]
    Subscribe1([WsSubscription; 1]),
    #[serde(rename = "UNSUBSCRIBE")]
    Unsubscribe(Box<[WsSubscription]>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WsSubscription {
    market: Atom,
    stream: WsStream,
}

impl WsSubscription {
    pub fn new(market: impl Into<Atom>, stream: WsStream) -> Self {
        let market = market.into();
        WsSubscription { market, stream }
    }
}

impl<A> From<(A, WsStream)> for WsSubscription
where
    A: Into<Atom>,
{
    fn from((market, stream): (A, WsStream)) -> Self {
        WsSubscription::new(market, stream)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WsStream {
    Depth,
    Depth100ms,
    Trade,
}

impl WsStream {
    const DEPTH: &'static str = "depth";
    const DEPTH_100MS: &'static str = "depth@100ms";
    const TRADE: &'static str = "trade";

    pub fn as_str(self) -> &'static str {
        match self {
            WsStream::Depth => Self::DEPTH,
            WsStream::Depth100ms => Self::DEPTH_100MS,
            WsStream::Trade => Self::TRADE,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            Self::DEPTH => Self::Depth,
            Self::DEPTH_100MS => Self::Depth100ms,
            Self::TRADE => Self::Trade,
            _ => None?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpstreamApiRequest<T> {
    pub id: u64,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UpstreamApiResult<T> {
    #[serde(rename = "result")]
    Ok(T),
    #[serde(rename = "error")]
    Err { code: i32, msg: String },
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum UpstreamWebsocketMessage<T> {
    Response(UpstreamWebsocketResponse<T>),
    Event(WsEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpstreamWebsocketResponse<T> {
    pub id: u64,
    #[serde(flatten)]
    pub payload: Option<UpstreamWebsocketResult<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UpstreamWebsocketResult<T> {
    #[serde(rename = "result")]
    Ok(T),
    #[serde(rename = "error")]
    Err { code: i32, msg: String },
}

impl<T> UpstreamWebsocketResult<T> {
    pub fn into_result(self) -> Result<T, (i32, String)> {
        match self {
            Self::Ok(result) => Ok(result),
            Self::Err { code, msg } => Err((code, msg)),
        }
    }
}

mod deser {
    use std::fmt;

    use serde::de::{self, MapAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{WsEvent, WsStream, WsSubscription};

    impl Serialize for WsSubscription {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut buffer = String::with_capacity(32);
            buffer.push_str(&self.market);
            buffer.push('@');
            buffer.push_str(self.stream.as_str());
            serializer.serialize_str(&buffer)
        }
    }

    impl<'de> Deserialize<'de> for WsSubscription {
        fn deserialize<D>(deserializer: D) -> Result<WsSubscription, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(WsSubscriptionVisitor)
        }
    }

    struct WsSubscriptionVisitor;

    impl<'de> Visitor<'de> for WsSubscriptionVisitor {
        type Value = WsSubscription;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string in format market@streamName")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let parse = |s: &str| -> Option<Self::Value> {
                let n = s.find('@')?;
                let stream = WsStream::from_str(&s[n + 1..])?;
                let market = s[..n].into();
                Some(WsSubscription { market, stream })
            };

            parse(value).ok_or_else(|| E::custom(format!("unrecognized input: {}", value)))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }
    }

    enum WsEventField {
        Stream,
        Data,
    }

    impl WsEventField {
        const STREAM: &'static str = "stream";
        const DATA: &'static str = "data";
        const FIELDS: &'static [&'static str] = &[WsEventField::STREAM, WsEventField::DATA];
    }

    struct WsEventFieldVisitor;

    impl<'de> Visitor<'de> for WsEventFieldVisitor {
        type Value = WsEventField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("`stream` of `data`")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(match value {
                WsEventField::STREAM => WsEventField::Stream,
                WsEventField::DATA => WsEventField::Data,
                _ => Err(de::Error::unknown_field(value, WsEventField::FIELDS))?,
            })
        }
    }

    impl<'de> Deserialize<'de> for WsEventField {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_identifier(WsEventFieldVisitor)
        }
    }

    struct WsEventVisitor;

    impl<'de> Visitor<'de> for WsEventVisitor {
        type Value = WsEvent;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("enum WsEvent")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut stream = None;
            let mut result = None;
            while let Some(key) = map.next_key()? {
                match key {
                    WsEventField::Stream => {
                        if stream.is_some() {
                            return Err(de::Error::duplicate_field(WsEventField::STREAM));
                        }
                        let x: WsSubscription = map.next_value()?;
                        stream = Some(x.stream);
                    }
                    WsEventField::Data => {
                        if result.is_some() {
                            return Err(de::Error::duplicate_field(WsEventField::DATA));
                        }
                        let stream = stream
                            .as_ref()
                            .ok_or_else(|| de::Error::missing_field(WsEventField::STREAM))?;
                        result = Some(match stream {
                            WsStream::Depth | WsStream::Depth100ms => {
                                WsEvent::OrderBookDiff(map.next_value()?)
                            }
                            WsStream::Trade => WsEvent::Trade(map.next_value()?),
                        });
                    }
                }
            }
            let result = result.ok_or_else(|| de::Error::missing_field(WsEventField::STREAM))?;
            Ok(result)
        }
    }

    impl<'de> Deserialize<'de> for WsEvent {
        fn deserialize<D>(deserializer: D) -> Result<WsEvent, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(WsEventVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_diff_depth_event() {
        let input = "{\
            \"e\":\"depthUpdate\",\
            \"E\":1612660810605,\
            \"s\":\"BTCUSDT\",\
            \"U\":8541958786,\
            \"u\":8541959197,\
            \"b\":[[\"39202.93000000\",\"0.00000000\"]],\
            \"a\":[[\"39202.93000000\",\"0.00000000\"]]\
        }";

        let _res = serde_json::from_str::<OrderBookDiffEvent>(input).unwrap();
    }

    #[test]
    fn decode_ws_event() {
        let input = "{\
            \"stream\":\"btcusdt@depth\",\
            \"data\":{\
                \"e\":\"depthUpdate\",\
                \"E\":1612660810605,\
                \"s\":\"BTCUSDT\",\
                \"U\":8541958786,\
                \"u\":8541959197,\
                \"b\":[[\"39202.93000000\",\"0.00000000\"]],\
                \"a\":[[\"39202.93000000\",\"0.00000000\"]]\
            }\
        }";

        let _res = serde_json::from_str::<WsEvent>(input).unwrap();
    }

    #[test]
    fn decode_response_ws_event() {
        let input = "{\
            \"stream\":\"btcusdt@depth\",\
            \"data\":{\
                \"e\":\"depthUpdate\",\
                \"E\":1612660810605,\
                \"s\":\"BTCUSDT\",\
                \"U\":8541958786,\
                \"u\":8541959197,\
                \"b\":[[\"39202.93000000\",\"0.00000000\"]],\
                \"a\":[[\"39202.93000000\",\"0.00000000\"]]\
            }\
        }";

        let _res = serde_json::from_str::<UpstreamWebsocketMessage<WsEvent>>(input).unwrap();
    }
}
