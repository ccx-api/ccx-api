use std::fmt::Display;

use ccx_api_lib::serde_util::none_as_empty_str;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;
use serde_json::value::RawValue;
use serde_repr::Deserialize_repr;

use super::order_book::OrderBookSnapshot;

pub type WsResult<T> = Result<T, WsErr>;

/// Gate WebSocket API response
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WsResponse {
    /// Request timestamp in seconds
    pub time: i64,
    /// Request ID extracted from the client request payload if client request has one
    pub id: Option<i64>,
    /// WebSocket channel
    pub event: Event,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Event {
    /// Check if connection to server is still alive
    Pong(WsResult<()>),
    /// Periodically notify about top bids and asks snapshot with limited levels
    OrderBook(EventInner<OrderBookSnapshot>),
}

impl<'de> Deserialize<'de> for WsResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct WsResponseInternal {
            time: i64,
            id: Option<i64>,
            channel: Channel,
            #[serde(with = "none_as_empty_str", default)]
            event: Option<EventKind>,
            error: Option<WsErr>,
            result: Option<Box<RawValue>>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum EventKind {
            Subscribe,
            Unsubscribe,
            Update,
        }

        #[derive(Deserialize)]
        enum Channel {
            #[serde(rename = "spot.pong")]
            Pong,
            #[serde(rename = "spot.order_book")]
            OrderBook,
        }

        let WsResponseInternal {
            time,
            id,
            channel,
            event,
            error,
            result,
        } = WsResponseInternal::deserialize(deserializer)?;

        let result = match (error, result) {
            (Some(e), _) => Err(e),
            (_, Some(ok)) => Ok(ok),
            _ => Ok(serde_json::from_str("{}").unwrap()),
        };
        let event = match (channel, event) {
            (Channel::Pong, _) => Ok(Event::Pong(result.map(|_| ()))),
            (Channel::OrderBook, Some(EventKind::Subscribe)) => {
                Ok(Event::OrderBook(EventInner::Subscribe(result.map(|_| ()))))
            }
            (Channel::OrderBook, Some(EventKind::Unsubscribe)) => Ok(Event::OrderBook(
                EventInner::Unsubscribe(result.map(|_| ())),
            )),
            (Channel::OrderBook, Some(EventKind::Update)) => {
                Ok(Event::OrderBook(EventInner::Update(match result {
                    Ok(json) => Ok(serde_json::from_str(json.get()).map_err(D::Error::custom)?),
                    Err(err) => Err(err),
                })))
            }
            (_, None) => Err(D::Error::missing_field("event")),
        }?;
        Ok(WsResponse { time, id, event })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum EventInner<T> {
    /// Server response to subscription request
    Subscribe(WsResult<()>),
    /// Server response to unsubscription request
    Unsubscribe(WsResult<()>),
    /// Server notification (update) with new info previously subscribed to
    Update(WsResult<T>),
}

/// Gate WebSocket API error
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WsErr {
    pub code: WsErrCode,
    pub message: String,
}

impl Display for WsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Represents error codes returned by the server.
#[derive(Debug, Clone, Copy, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum WsErrCode {
    /// Invalid request body format.
    InvalidRequestBody = 1,
    /// Invalid argument provided.
    InvalidArgument = 2,
    /// Server-side error happened.
    ServerError = 3,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use similar_asserts::assert_eq;

    use super::Event;
    use crate::websocket::order_book::OrderBookSnapshot;
    use crate::websocket::response::EventInner;
    use crate::websocket::response::WsErr;
    use crate::websocket::response::WsErrCode::ServerError;
    use crate::websocket::response::WsResponse;

    #[test]
    fn deserialize_pong_success() {
        let json = r#"{
  "time": 1545404023,
  "channel": "spot.pong",
  "event": "",
  "error": null,
  "result": null
}"#;
        let expected = WsResponse::new(1545404023, Event::Pong(Ok(())));
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    #[test]
    fn deserialize_pong_error() {
        let json = r#"{
  "time": 1545404023,
  "channel": "spot.pong",
  "event": "",
  "error": {
    "code": 3,
    "message": "Server side error"
  },
  "result": null
}"#;
        let expected = WsResponse::new(
            1545404023,
            Event::Pong(Err(WsErr {
                code: ServerError,
                message: "Server side error".into(),
            })),
        );
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    #[test]
    fn deserialize_order_book() {
        let json = r#"{
  "time": 1545404023,
  "channel": "spot.order_book",
  "event": "update",
  "result": {
    "t": 1606295412123,
    "lastUpdateId": 48791820,
    "s": "BTC_USDT",
    "bids": [
      ["19079.55", "0.0195"],
      ["19079.07", "0.7341"],
      ["19076.23", "0.00011808"],
      ["19073.9", "0.105"],
      ["19068.83", "0.1009"]
    ],
    "asks": [
      ["19080.24", "0.1638"],
      ["19080.91", "0.1366"],
      ["19080.92", "0.01"],
      ["19081.29", "0.01"],
      ["19083.8", "0.097"]
    ]
  }
}"#;
        let expected = WsResponse::new(
            1545404023,
            Event::OrderBook(EventInner::Update(Ok(OrderBookSnapshot {
                update_time_ms: 1606295412123,
                last_update_id: 48791820,
                currency_pair: "BTC_USDT".into(),
                bids: vec![
                    (dec!(19079.55), dec!(0.0195)).into(),
                    (dec!(19079.07), dec!(0.7341)).into(),
                    (dec!(19076.23), dec!(0.00011808)).into(),
                    (dec!(19073.9), dec!(0.105)).into(),
                    (dec!(19068.83), dec!(0.1009)).into(),
                ],
                asks: vec![
                    (dec!(19080.24), dec!(0.1638)).into(),
                    (dec!(19080.91), dec!(0.1366)).into(),
                    (dec!(19080.92), dec!(0.01)).into(),
                    (dec!(19081.29), dec!(0.01)).into(),
                    (dec!(19083.8), dec!(0.097)).into(),
                ],
            }))),
        );
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    impl WsResponse {
        fn new(time: i64, event: Event) -> Self {
            Self {
                time,
                event,
                id: None,
            }
        }
    }
}
