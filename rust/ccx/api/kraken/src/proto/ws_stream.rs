use ccx_api_lib::Decimal;
use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;

use crate::util::{Ask, Bid, OrderBook};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum WsEvent {
    // Public
    Trade(TradeEvent),
    SnapOrderBook(SnapOrderBookEvent),
    DiffOrderBook(DiffOrderBookEvent),

    // General
    SystemStatus(SystemStatusEvent),
    SubscriptionStatus(SubscriptionStatusEvent),
    Pong(PongEvent),
    Heartbeat(HeartbeatEvent),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TradeEvent {
    pub channel_id: u64,
    pub data: Vec<TradeData>,
    pub channel_name: String,
    pub pair: Atom,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TradeData {
    pub price: Decimal,
    pub volume: Decimal,
    pub time: Decimal,
    pub side: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    pub misc: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SnapOrderBookEvent {
    pub channel_id: u64,
    pub data: SnapOrderBookData,
    pub channel_name: String,
    pub pair: Atom,
}

impl Into<OrderBook> for SnapOrderBookEvent {
    fn into(self) -> OrderBook {
        OrderBook {
            last_checksum: Default::default(),
            bids: self.data.bids.into(),
            asks: self.data.asks.into(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SnapOrderBookData {
    #[serde(rename = "bs")]
    pub bids: Vec<Bid>,
    #[serde(rename = "as")]
    pub asks: Vec<Ask>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DiffOrderBookEvent {
    pub channel_id: u64,
    pub data: DiffOrderBookData,
    pub channel_name: String,
    pub pair: Atom,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DiffOrderBookData {
    #[serde(default, rename = "b")]
    pub bids: Option<Vec<Bid>>,
    #[serde(default, rename = "a")]
    pub asks: Option<Vec<Ask>>,
    #[serde(default, rename = "c")]
    pub checksum: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct PongEvent {
    pub reqid: u64,
    pub event: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct HeartbeatEvent {
    pub event: HeartbeatEventName,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatEventName {
    Heartbeat,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SystemStatusEvent {
    #[serde(default, rename = "connectionID")]
    pub connection_id: Option<u64>,
    pub event: String,
    pub status: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SubscriptionStatusEvent {
    #[serde(default, rename = "channelID")]
    pub channel_id: Option<u64>,
    #[serde(default, rename = "channelName")]
    pub channel_name: Option<String>,
    #[serde(default)]
    pub pair: Option<Atom>,
    pub subscription: SubscriptionStatusEventSubscription,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SubscriptionStatusEventSubscription {
    #[serde(default)]
    pub depth: Option<u32>,
    #[serde(default)]
    pub interval: Option<u32>,
    #[serde(default)]
    pub maxratecount: Option<u32>,
    #[serde(default)]
    pub reqid: Option<u64>,
    pub name: String,
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum WsCommand {
    Subscribe(WsSubscription),
    Unsubscribe(WsSubscription),
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct WsSubscription {
    pair: Vec<Atom>,
    #[serde(rename = "subscription")]
    stream: WsStream,
}

impl WsSubscription {
    pub fn new(pair: impl Into<Vec<Atom>>, stream: WsStream) -> Self {
        let pair = pair.into();
        WsSubscription { pair, stream }
    }
}

impl<A> From<(A, WsStream)> for WsSubscription
where
    A: Into<Vec<Atom>>,
{
    fn from((pair, stream): (A, WsStream)) -> Self {
        WsSubscription::new(pair, stream)
    }
}

#[derive(Debug, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WsStreamBookParams {
    pub depth: u16,
}

#[derive(Debug, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WsStreamTradeParams {}

#[derive(Debug, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum WsStream {
    Book(WsStreamBookParams),
    Trade(WsStreamTradeParams),
    // Ticker(WsStreamTickerParams)
}

impl WsStream {
    const BOOK: &'static str = "book";
    const TRADE: &'static str = "trade";
    // const TICKER: &'static str = "ticker";

    pub fn as_str(self) -> &'static str {
        match self {
            WsStream::Book(_) => Self::BOOK,
            WsStream::Trade(_) => Self::TRADE,
            // WsStream::Ticker(_) => Self::TICKER,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            Self::BOOK => Self::Book(WsStreamBookParams { depth: 10 }),
            Self::TRADE => Self::Trade(WsStreamTradeParams {}),
            // Self::TICKER => Self::Ticker(WsStreamBookParams { depth: 10 }),
            _ => None?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpstreamApiRequest<T> {
    pub reqid: u64,
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UpstreamWebsocketResponse<T> {
    pub reqid: u64,
    pub event: String,
    pub status: String,
    #[serde(flatten)]
    pub payload: UpstreamWebsocketResult<T>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum UpstreamWebsocketResult<T> {
    Err {
        #[serde(rename = "errorMessage")]
        error_message: String,
    },
    Ok(T),
}

impl<T> UpstreamWebsocketResult<T> {
    pub fn into_result(self) -> Result<T, String> {
        match self {
            Self::Ok(result) => Ok(result),
            Self::Err { error_message } => Err(format!("Kraken WS Error! {}", error_message)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_decimal::Decimal;

    #[test]
    fn test_decode_trade_event() {
        let input = r#"[
            0,
            [
                [
                    "5541.20000",
                    "0.15850568",
                    "1534614057.321597",
                    "s",
                    "l",
                    ""
                ],
                [
                    "6060.00000",
                    "0.02455000",
                    "1534614057.324998",
                    "b",
                    "l",
                    ""
                ]
            ],
            "trade",
            "XBT/USD"
        ]"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::Trade(e)) => {
                assert_eq!(e.channel_id, 0);
                assert_eq!(e.data.len(), 2);
                assert_eq!(
                    e.data[0].price,
                    Decimal::from_str_exact("5541.20000").unwrap()
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_snap_order_book_event() {
        let input = r#"[
            2304,
            {
                "as": [
                    ["1226.20000","0.10000000","1656410437.680548"],
                    ["1269.00000","0.20000000","1656406196.871239"]
                ],
                "bs": [
                    ["1225.20000","0.10000000","1656410437.680548"],
                    ["1268.00000","0.20000000","1656406196.871239"]
                ]
            },
            "book-100",
            "ETH/USDT"
        ]"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::SnapOrderBook(e)) => {
                assert_eq!(e.channel_id, 2304);
                assert_eq!(e.data.asks.len(), 2);
                assert_eq!(
                    e.data.asks[0].price,
                    Decimal::from_str_exact("1226.20000").unwrap()
                );
                assert_eq!(e.data.bids.len(), 2);
                assert_eq!(
                    e.data.bids[0].price,
                    Decimal::from_str_exact("1225.20000").unwrap()
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_diff_order_book_event() {
        let input = r#"[
            2304,
            {
                "a": [
                    ["1226.20000","0.10000000","1656410437.680548"],
                    ["1269.00000","0.20000000","1656406196.871239","r"]
                ],
                "c":"2474294632"
            },
            "book-100",
            "ETH/USDT"
        ]"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::DiffOrderBook(e)) => {
                assert_eq!(e.channel_id, 2304);
                assert_eq!(e.data.asks.as_ref().unwrap().len(), 2);
                assert_eq!(
                    e.data.asks.as_ref().unwrap()[0].price,
                    Decimal::from_str_exact("1226.20000").unwrap()
                );
                assert_eq!(e.data.bids, None);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_system_status_event() {
        let input = r#"{
            "connectionID":16104859528062827651,
            "event":"systemStatus",
            "status":"online",
            "version":"1.9.0"
        }"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::SystemStatus(e)) => {
                assert_eq!(e.connection_id, Some(16104859528062827651));
                assert_eq!(e.event, "systemStatus");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_subscription_status_event() {
        let input = r#"{
            "channelID":2288,
            "channelName":"book-1000",
            "event":"subscriptionStatus",
            "pair":"XBT/USDT",
            "reqid":1,
            "status":"subscribed",
            "subscription":{"depth":1000,"name":"book"}
        }"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Response(e) => match e.payload {
                UpstreamWebsocketResult::Ok(WsEvent::SubscriptionStatus(ss)) => {
                    assert_eq!(e.event, "subscriptionStatus");

                    assert_eq!(ss.channel_id, Some(2288));
                    assert_eq!(ss.subscription.name, "book");
                }
                _ => {}
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_subscription_status_event_error() {
        let input = r#"{
            "errorMessage":"Currency pair not in ISO 4217-A3 format XBTUSDT",
            "event":"subscriptionStatus",
            "pair":"XBTUSDT",
            "reqid":1,
            "status":"error",
            "subscription":{"depth":10,"name":"book"}
        }"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Response(e) => {
                assert_eq!(e.event, "subscriptionStatus");
                assert_eq!(e.status, "error");

                assert_eq!(
                    e.payload.into_result(),
                    Err("Kraken WS Error! Currency pair not in \
                        ISO 4217-A3 format XBTUSDT"
                        .into())
                );
            }
            _ => unreachable!(),
        }
    }
}
