use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use serde::Deserialize;

use super::DetailOrderBookEvent;
use super::LiveOrderEvent;
use super::LiveTradeEvent;
use super::OrderBookEvent;
use super::WsStream;
use crate::ws_stream::channel_from_raw;
use crate::ws_stream::LiveOrderEventType;
use crate::Atom;

/// Internal type of event returned from bitstamp subscription.
#[derive(Clone, Debug, Deserialize, From)]
#[serde(try_from = "InnerEvent")]
pub(crate) enum Event {
    System(SystemEvent),
    Client(WsEvent),
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct InnerEvent {
    event: EventType,
    #[serde(default)]
    channel: String,
    #[serde(default)]
    data: serde_json::Value,
}

impl TryFrom<InnerEvent> for Event {
    type Error = DeserializeError;

    fn try_from(value: InnerEvent) -> Result<Self, Self::Error> {
        Ok(match value.event {
            EventType::System(ev) => SystemEvent::try_new(ev, value.channel, value.data)?.into(),
            EventType::Client(ev) => WsEvent::try_new(ev, value.channel, value.data)?.into(),
        })
    }
}

/// Wrapper to distinct between protocol\system events and actual data.
#[derive(Clone, Debug, From, Deserialize)]
#[serde(untagged)]
enum EventType {
    /// Events used by the control flow.
    ///
    /// Such events start with prefix `"bts:"`.
    System(SystemEventType),
    /// Events that should be exposed to the end consumer.
    Client(ClientEventType),
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) enum ClientEventType {
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "order_created")]
    OrderCreated,
    #[serde(rename = "order_changed")]
    OrderChanged,
    #[serde(rename = "order_deleted")]
    OrderDeleted,
    #[serde(rename = "trade")]
    Trade,
}

/// Actual data returned from bitstamp websockets that may be interesting to the
/// end user.
#[derive(Clone, Debug)]
pub enum WsEvent {
    LiveTrade {
        pair: Atom,
        data: LiveTradeEvent,
    },
    LiveOrders {
        pair: Atom,
        data: LiveOrderEvent,
    },
    OrderBook {
        pair: Atom,
        data: OrderBookEvent,
    },
    DetailOrderBook {
        pair: Atom,
        data: DetailOrderBookEvent,
    },
    DiffOrderBook {
        pair: Atom,
        data: OrderBookEvent,
    },
}

impl WsEvent {
    pub(crate) fn try_new(
        ev: ClientEventType,
        channel: String,
        data: serde_json::Value,
    ) -> Result<Self, DeserializeError> {
        let (stream, pair) = channel_from_raw(&channel)
            .ok_or_else(|| DeserializeError::InvalidChannelName(channel))?;
        let event = match (ev, stream) {
            (ClientEventType::Data, WsStream::OrderBook) => WsEvent::OrderBook {
                pair,
                data: serde_json::from_value(data)?,
            },
            (ClientEventType::Data, WsStream::DiffOrderBook) => WsEvent::DiffOrderBook {
                pair,
                data: serde_json::from_value(data)?,
            },
            (ClientEventType::Data, WsStream::DetailOrderBook) => WsEvent::DetailOrderBook {
                pair,
                data: serde_json::from_value(data)?,
            },
            (ClientEventType::OrderCreated, WsStream::LiveOrders) => {
                let mut event: LiveOrderEvent = serde_json::from_value(data)?;
                event.event_type = LiveOrderEventType::OrderCreated;
                WsEvent::LiveOrders { pair, data: event }
            }
            (ClientEventType::OrderChanged, WsStream::LiveOrders) => {
                let mut event: LiveOrderEvent = serde_json::from_value(data)?;
                event.event_type = LiveOrderEventType::OrderChanged;
                WsEvent::LiveOrders { pair, data: event }
            }
            (ClientEventType::OrderDeleted, WsStream::LiveOrders) => {
                let mut event: LiveOrderEvent = serde_json::from_value(data)?;
                event.event_type = LiveOrderEventType::OrderDeleted;
                WsEvent::LiveOrders { pair, data: event }
            }
            (ev, stream) => return Err(DeserializeError::InvalidEventAndChannel(ev, stream)),
        };

        Ok(event)
    }
}

/// Complete set of system event types.
#[derive(Clone, Debug, Deserialize)]
enum SystemEventType {
    #[serde(rename = "bts:heartbeat")]
    Heartbeat,
    #[serde(rename = "bts:request_reconnect")]
    ReconnectRequest,
    #[serde(rename = "bts:subscription_succeeded")]
    SubscriptionSucceeded,
    #[serde(rename = "bts:error")]
    Error,
}

/// Complete set of system events with corresponding data.
#[derive(Clone, Debug)]
pub(crate) enum SystemEvent {
    Heartbeat,
    ReconnectRequest,
    SubscriptionSucceeded { channel: (WsStream, Atom) },
    Error { channel: String, data: WsError },
}

impl SystemEvent {
    fn try_new(
        ev: SystemEventType,
        channel: String,
        data: serde_json::Value,
    ) -> Result<Self, DeserializeError> {
        let event = match ev {
            SystemEventType::Heartbeat => Self::Heartbeat,
            SystemEventType::ReconnectRequest => Self::ReconnectRequest,
            SystemEventType::SubscriptionSucceeded => {
                let channel = channel_from_raw(&channel)
                    .ok_or_else(|| DeserializeError::InvalidChannelName(channel))?;
                Self::SubscriptionSucceeded { channel }
            }
            SystemEventType::Error => Self::Error {
                channel,
                data: serde_json::from_value(data)?,
            },
        };

        Ok(event)
    }
}

/// Error returned but Bitstamp through websockets.
#[derive(Clone, Debug, Deserialize)]
pub struct WsError {
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: String,
}

/// Deserialization Error for Event
#[derive(Debug, Display, Error, From)]
pub(crate) enum DeserializeError {
    Serde(serde_json::Error),
    #[display(fmt = "Invalid channel")]
    #[from(ignore)]
    InvalidChannelName(#[error(not(source))] String),
    #[display(fmt = "Invalid combination of `event`: {:?} and `stream`:{:?}", _0, _1)]
    InvalidEventAndChannel(ClientEventType, WsStream),
}
