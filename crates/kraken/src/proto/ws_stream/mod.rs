use serde::Deserialize;
use serde::Serialize;
use string_cache::DefaultAtom as Atom;

mod heartbeat;
mod orderbook_diff;
mod orderbook_snap;
mod pong;
mod subscription_status;
mod system_status;
mod trade;

pub use heartbeat::*;
pub use orderbook_diff::*;
pub use orderbook_snap::*;
pub use pong::*;
pub use subscription_status::*;
pub use system_status::*;
pub use trade::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum WsEvent {
    // Public
    Trade(Trade),
    OrderBookSnap(OrderBookSnap),
    OrderBookDiff(OrderBookDiff),

    // General
    SystemStatus(SystemStatus),
    SubscriptionStatus(SubscriptionStatus),
    Pong(Pong),
    Heartbeat(Heartbeat),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum WsCommand {
    Subscribe(WsSubscription),
    Unsubscribe(WsSubscription),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WsSubscription {
    pub pair: Vec<Atom>,
    #[serde(rename = "subscription")]
    pub stream: WsStream,
}

impl WsSubscription {
    fn new(pair: impl Into<Vec<Atom>>, stream: WsStream) -> Self {
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WsStreamBookParams {
    pub depth: u16,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WsStreamTradeParams {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum WsStream {
    Book(WsStreamBookParams),
    Trade(WsStreamTradeParams),
    // Ticker(WsStreamTickerParams)
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UpstreamApiRequest<T> {
    pub reqid: u64,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum UpstreamApiResult<T> {
    #[serde(rename = "result")]
    Ok(T),
    #[serde(rename = "error")]
    Err { code: i32, msg: String },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum UpstreamWebsocketMessage<T> {
    Response(UpstreamWebsocketResponse<T>),
    Event(WsEvent),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UpstreamWebsocketResponse<T> {
    pub reqid: u64,
    pub event: String,
    pub status: String,
    #[serde(flatten)]
    pub payload: UpstreamWebsocketResult<T>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
