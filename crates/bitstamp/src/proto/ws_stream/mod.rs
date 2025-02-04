mod detail_order_book;
mod live_order;
mod live_trade;
mod order_book;
mod request;
mod response;

use serde::Deserialize;
use serde::Serialize;
use string_cache::DefaultAtom as Atom;

pub use self::detail_order_book::*;
pub use self::live_order::*;
pub use self::live_trade::*;
pub use self::order_book::*;
pub use self::request::*;
pub use self::response::*;

/// Represents subscription to certain `channel`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct WsSubscription {
    #[serde(with = "channel")]
    pub(crate) channel: (WsStream, Atom),
}

impl WsSubscription {
    /// Constructs new [`WsSubscription`] from [`WsStream`] and pair name.
    pub fn new(stream: WsStream, pair: impl Into<Atom>) -> Self {
        WsSubscription {
            channel: (stream, pair.into()),
        }
    }
}

impl<A> From<(WsStream, A)> for WsSubscription
where
    A: Into<Atom>,
{
    fn from((stream, pair): (WsStream, A)) -> Self {
        WsSubscription::new(stream, pair)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WsStream {
    LiveTrades,
    LiveOrders,
    OrderBook,
    DetailOrderBook,
    DiffOrderBook,
}

impl WsStream {
    const LIVE_TRADES: &'static str = "live_trades";
    const LIVE_ORDERS: &'static str = "live_orders";
    const ORDER_BOOK: &'static str = "order_book";
    const DETAIL_ORDER_BOOK: &'static str = "detail_order_book";
    const DIFF_ORDER_BOOK: &'static str = "diff_order_book";

    pub fn as_str(self) -> &'static str {
        match self {
            WsStream::LiveTrades => Self::LIVE_TRADES,
            WsStream::LiveOrders => Self::LIVE_ORDERS,
            WsStream::OrderBook => Self::ORDER_BOOK,
            WsStream::DetailOrderBook => Self::DETAIL_ORDER_BOOK,
            WsStream::DiffOrderBook => Self::DIFF_ORDER_BOOK,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            Self::LIVE_TRADES => Self::LiveTrades,
            Self::LIVE_ORDERS => Self::LiveOrders,
            Self::ORDER_BOOK => Self::OrderBook,
            Self::DETAIL_ORDER_BOOK => Self::DetailOrderBook,
            Self::DIFF_ORDER_BOOK => Self::DiffOrderBook,
            _ => None?,
        })
    }
}

fn channel_from_raw(value: &str) -> Option<(WsStream, Atom)> {
    let n = value.rfind('_')?;
    let stream = WsStream::from_str(&value[..n])?;
    let pair = value[n + 1..].into();
    Some((stream, pair))
}

mod channel {
    use std::fmt;

    use serde::de::Visitor;
    use serde::de::{self};
    use serde::Deserializer;
    use serde::Serializer;

    use super::Atom;
    use super::WsStream;

    pub(super) fn serialize<S>(pair: &(WsStream, Atom), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}_{}", pair.0.as_str(), &pair.1))
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<(WsStream, Atom), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WsSubscriptionVisitor)
    }

    struct WsSubscriptionVisitor;

    impl<'de> Visitor<'de> for WsSubscriptionVisitor {
        type Value = (WsStream, Atom);

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string in format {channel_name}_{pair}")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            super::channel_from_raw(value)
                .ok_or_else(|| E::custom(format!("unrecognized input: {}", value)))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }
    }
}
