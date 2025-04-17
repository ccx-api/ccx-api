use serde::Deserialize;
use serde::Deserializer;
use serde::de::DeserializeOwned;
use smallvec::SmallVec;

use super::{OrderBookChannelSubscribed, OrderBookSnapshot, OrderBookUpdate, StatusUpdate};

pub type ReqId = i64;
pub type WsResult<T> = Result<T, WsErr>;

/// Gate WebSocket API response
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WsResponse {
    /// Request ID extracted from the client request payload if client request has one
    pub req_id: Option<ReqId>,
    /// WebSocket channel
    pub event: Event,
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct PongResponse {
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(try_from = "ChannelResponseRaw<S, U>")]
pub enum ChannelResponse<S, U> {
    Snapshot(S),
    Update(U),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "channel")]
#[serde(rename_all = "snake_case")]
pub enum ChannelsSubscribed {
    #[serde(rename = "book")]
    OrderBook(OrderBookChannelSubscribed),
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Event {
    Heartbeat,
    /// Check if connection to server is still alive
    Pong(WsResult<PongResponse>),
    Subscribe(WsResult<ChannelsSubscribed>),
    // TODO: channels unsubsribe could have different schema from subscription
    Unsubscribe(WsResult<ChannelsSubscribed>),
    Status(ChannelResponse<StatusUpdate, StatusUpdate>),
    /// Periodically notify about top bids and asks snapshot with limited levels
    OrderBook(ChannelResponse<OrderBookSnapshot, OrderBookUpdate>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum MethodRaw {
    Pong,
    Subscribe,
    Unsubscribe,
}

#[derive(Deserialize)]
struct WsResponseMethod {
    method: MethodRaw,
    result: Option<serde_json::Value>,
    error: Option<String>,
    // success: Option<bool>,
    // time_in: String,
    // time_out: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ChannelResponseRaw<S, U> {
    Snapshot { data: SmallVec<[S; 1]> },
    Update { data: SmallVec<[U; 1]> },
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum InternalError {
    UnexpectedDataLength,
}

impl<S, U> TryFrom<ChannelResponseRaw<S, U>> for ChannelResponse<S, U> {
    type Error = InternalError;

    fn try_from(value: ChannelResponseRaw<S, U>) -> Result<Self, Self::Error> {
        match value {
            ChannelResponseRaw::Snapshot { mut data } => {
                if data.len() == 1 {
                    Ok(ChannelResponse::Snapshot(
                        data.pop().expect("len check above"),
                    ))
                } else {
                    Err(InternalError::UnexpectedDataLength)
                }
            }
            ChannelResponseRaw::Update { mut data } => {
                if data.len() == 1 {
                    Ok(ChannelResponse::Update(
                        data.pop().expect("len check above"),
                    ))
                } else {
                    Err(InternalError::UnexpectedDataLength)
                }
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "channel")]
#[serde(rename_all = "snake_case")]
enum WsResponseChannel {
    Book(ChannelResponse<OrderBookSnapshot, OrderBookUpdate>),
    Status(ChannelResponse<StatusUpdate, StatusUpdate>),
    Heartbeat,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum WsResponseRawInner {
    Method(WsResponseMethod),
    Channel(WsResponseChannel),
}

#[derive(Deserialize)]
struct WsResponseRaw {
    req_id: Option<ReqId>,
    #[serde(flatten)]
    inner: WsResponseRawInner,
}

impl<'de> Deserialize<'de> for WsResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw_response = WsResponseRaw::deserialize(deserializer)?;

        let event = match raw_response.inner {
            WsResponseRawInner::Method(method) => {
                if let Some(message) = method.error {
                    fn error_fn<T>(message: String) -> WsResult<T> {
                        WsResult::Err(WsErr::Api { message })
                    }

                    let event = match method.method {
                        MethodRaw::Pong => Event::Pong(error_fn(message)),
                        MethodRaw::Subscribe => Event::Subscribe(error_fn(message)),
                        MethodRaw::Unsubscribe => Event::Unsubscribe(error_fn(message)),
                    };

                    return Ok(WsResponse {
                        req_id: raw_response.req_id,
                        event,
                    });
                };

                fn parse_result<T, E>(result: Option<serde_json::Value>) -> Result<T, E>
                where
                    T: DeserializeOwned,
                    E: serde::de::Error,
                {
                    serde_json::from_value(result.unwrap_or_default())
                        .map_err(serde::de::Error::custom)
                }

                tracing::debug!(?method.result, ?method.method, "parsing result");

                match method.method {
                    MethodRaw::Pong => Event::Pong(Ok(parse_result::<_, D::Error>(method.result)
                        // in case there is no warnings the pong result will be empty
                        .unwrap_or_default())),
                    MethodRaw::Subscribe => Event::Subscribe(Ok(parse_result(method.result)?)),
                    MethodRaw::Unsubscribe => Event::Unsubscribe(Ok(parse_result(method.result)?)),
                }
            }
            WsResponseRawInner::Channel(channel) => match channel {
                WsResponseChannel::Heartbeat => Event::Heartbeat,
                WsResponseChannel::Status(channel) => Event::Status(channel),
                WsResponseChannel::Book(channel) => Event::OrderBook(channel),
            },
        };

        Ok(WsResponse {
            req_id: raw_response.req_id,
            event,
        })
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
#[derive(Debug, Clone, Deserialize, derive_more::Display, derive_more::Error, PartialEq, Eq)]
pub enum WsErr {
    Api { message: String },
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use similar_asserts::assert_eq;

    use super::Event;
    use crate::api::spot::market::Status;
    use crate::api_ws::{
        ChannelResponse, OrderBookSnapshot, OrderBookUpdate, PongResponse, PriceAndQty,
        StatusUpdate,
    };
    // use crate::api_ws::order_book::OrderBookSnapshot;
    use crate::api_ws::response::EventInner;
    use crate::api_ws::response::WsErr;
    use crate::api_ws::response::WsResponse;

    #[test]
    fn deserialize_status() {
        let json = r#"{
            "channel":"status",
            "type":"update",
            "data":[
                {
                    "version":"2.0.9",
                    "system":"online",
                    "api_version":"v2",
                    "connection_id":941764291031323042
                }
            ]
        }"#;
        let expected = WsResponse {
            req_id: None,
            event: Event::Status(ChannelResponse::Update(StatusUpdate {
                version: "2.0.9".to_string(),
                system: Status::Online,
                api_version: "v2".to_string(),
            })),
        };
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    #[test]
    fn deserialize_pong_success() {
        let json = r#"{
            "method": "pong",
            "req_id": 101,
            "time_in": "2023-09-24T14:10:23.799685Z",
            "time_out": "2023-09-24T14:10:23.799703Z"
        }"#;
        let expected = WsResponse {
            req_id: Some(101),
            event: Event::Pong(Ok(PongResponse { warnings: None })),
        };
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    #[test]
    fn deserialize_pong_error() {
        let json = r#"{
            "method": "pong",
            "req_id": 103,
            "success": false,
            "error": "Failed to execute ping",
            "time_in": "2023-09-24T14:10:23.799685Z",
            "time_out": "2023-09-24T14:10:23.799703Z"
        }"#;
        let expected = WsResponse {
            req_id: Some(103),
            event: Event::Pong(Err(WsErr::Api {
                message: "Failed to execute ping".to_string(),
            })),
        };
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    #[test]
    fn deserialize_order_book_snapshot() {
        let json = r#"{
            "channel": "book",
            "type": "update",
            "data": [
                {
                    "symbol": "MATIC/USD",
                    "bids": [
                        {
                            "price": 0.5657,
                            "qty": 1098.3947558
                        }
                    ],
                    "asks": [],
                    "checksum": 2114181697,
                    "timestamp": "2023-10-06T17:35:55.440295Z"
                }
            ]
        }"#;
        let expected = WsResponse {
            req_id: None,
            event: Event::OrderBook(ChannelResponse::Update(OrderBookUpdate {
                checksum: 2114181697,
                symbol: "MATIC/USD".into(),
                bids: vec![PriceAndQty {
                    price: dec!(0.5657),
                    qty: dec!(1098.3947558),
                }]
                .into(),
                asks: vec![].into(),
                timestamp: "2023-10-06T17:35:55.440295Z".to_string(),
            })),
        };
        let actual = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(actual).unwrap());
    }
}
