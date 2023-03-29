use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;

use super::*;
use crate::util::{Ask, Bid};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WsEvent {
    AggTrade(AggTradeEvent),
    DiffOrderBook(DiffOrderBookEvent),
    Kline(KlineEvent),
    Trade(TradeEvent),
}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct AccountUpdateEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    m: u64,
//    t: u64,
//    b: u64,
//    s: u64,
//
//    #[serde(rename = "T")] t_ignore: bool,
//    #[serde(rename = "W")] w_ignore: bool,
//    #[serde(rename = "D")] d_ignore: bool,
//
//    #[serde(rename = "B")] pub balance: Vec<EventBalance>,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct EventBalance {
//    #[serde(rename = "a")] pub asset: String,
//    #[serde(rename = "f")] pub free: String,
//    #[serde(rename = "l")] pub locked: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct OrderTradeEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    #[serde(rename = "s")] pub symbol: String,
//
//    #[serde(rename = "c")] pub new_client_order_id: String,
//
//    #[serde(rename = "S")] pub side: String,
//
//    #[serde(rename = "o")] pub order_type: String,
//
//    #[serde(rename = "f")] pub time_in_force: String,
//
//    #[serde(rename = "q")] pub qty: String,
//
//    #[serde(rename = "p")] pub price: String,
//
//    #[serde(skip_serializing, rename = "P")] pub p_ignore: String,
//
//    #[serde(skip_serializing, rename = "F")] pub f_ignore: String,
//
//    #[serde(skip_serializing)] pub g: i32,
//
//    #[serde(skip_serializing, rename = "C")] pub c_ignore: Option<String>,
//
//    #[serde(rename = "x")] pub execution_type: String,
//
//    #[serde(rename = "X")] pub order_status: String,
//
//    #[serde(rename = "r")] pub order_reject_reason: String,
//
//    #[serde(rename = "i")] pub order_id: u64,
//
//    #[serde(rename = "l")] pub qty_last_filled_trade: String,
//
//    #[serde(rename = "z")] pub accumulated_qty_filled_trades: String,
//
//    #[serde(rename = "L")] pub price_last_filled_trade: String,
//
//    #[serde(rename = "n")] pub commission: String,
//
//    #[serde(skip_serializing, rename = "N")] pub asset_commisioned: Option<String>,
//
//    #[serde(rename = "T")] pub trade_order_time: u64,
//
//    #[serde(rename = "t")] pub trade_id: i64,
//
//    #[serde(skip_serializing, rename = "I")] pub i_ignore: u64,
//
//    #[serde(skip_serializing)] pub w: bool,
//
//    #[serde(rename = "m")] pub is_buyer_maker: bool,
//
//    #[serde(skip_serializing, rename = "M")] pub m_ignore: bool,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct DayTickerEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    #[serde(rename = "s")] pub symbol: String,
//
//    #[serde(rename = "p")] pub price_change: String,
//
//    #[serde(rename = "P")] pub price_change_percent: String,
//
//    #[serde(rename = "w")] pub average_price: String,
//
//    #[serde(rename = "x")] pub prev_close: String,
//
//    #[serde(rename = "c")] pub current_close: String,
//
//    #[serde(rename = "Q")] pub current_close_qty: String,
//
//    #[serde(rename = "b")] pub best_bid: String,
//
//    #[serde(rename = "B")] pub best_bid_qty: String,
//
//    #[serde(rename = "a")] pub best_ask: String,
//
//    #[serde(rename = "A")] pub best_ask_qty: String,
//
//    #[serde(rename = "o")] pub open: String,
//
//    #[serde(rename = "h")] pub high: String,
//
//    #[serde(rename = "l")] pub low: String,
//
//    #[serde(rename = "v")] pub volume: String,
//
//    #[serde(rename = "q")] pub quote_volume: String,
//
//    #[serde(rename = "O")] pub open_time: u64,
//
//    #[serde(rename = "C")] pub close_time: u64,
//
//    #[serde(rename = "F")] pub first_trade_id: u64,
//
//    #[serde(rename = "L")] pub last_trade_id: u64,
//
//    #[serde(rename = "n")] pub num_trades: u64,
//}
//

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct AggTradeEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "a")]
    pub id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub qty: Decimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TradeEvent {
    /// Event type.
    #[serde(skip, rename = "e")]
    pub event_type: (),
    /// Event time.
    #[serde(rename = "E")]
    pub event_time: u64,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: Atom,
    /// Trade ID.
    #[serde(rename = "t")]
    pub id: u64,
    /// Price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// Quantity.
    #[serde(rename = "q")]
    pub qty: Decimal,
    /// Buyer order ID.
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    /// Seller order ID.
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    /// Trade time.
    #[serde(rename = "T")]
    pub time: u64,
    /// Is the buyer the market maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    /// Ignore.
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct KlineEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "k")]
    pub kline: WSKline,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct WSKline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "i")]
    pub interval: ChartInterval,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "V")]
    pub active_buy_volume: Decimal,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: Decimal,
    #[serde(skip, rename = "B")]
    pub ignore: (),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct MiniTickerEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TickerEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "p")]
    pub price_change: Decimal,
    #[serde(rename = "P")]
    pub price_change_percent: Decimal,
    #[serde(rename = "w")]
    pub weighted_avg_price: Decimal,
    #[serde(rename = "x")]
    pub first_trade: Decimal,
    #[serde(rename = "c")]
    pub last_price: Decimal,
    #[serde(rename = "Q")]
    pub last_qty: Decimal,
    #[serde(rename = "b")]
    pub best_bid_price: Decimal,
    #[serde(rename = "B")]
    pub best_bid_qty: Decimal,
    #[serde(rename = "a")]
    pub best_ask_price: Decimal,
    #[serde(rename = "A")]
    pub best_ask_qty: Decimal,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "O")]
    pub stats_open_time: u64,
    #[serde(rename = "C")]
    pub stats_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DiffOrderBookEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
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
                                WsEvent::DiffOrderBook(map.next_value()?)
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

        let _res = serde_json::from_str::<DiffOrderBookEvent>(input).unwrap();
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
