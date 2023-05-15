use serde::{Deserialize, Serialize};

use crate::Atom;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionStatus {
    #[serde(default, rename = "channelID")]
    pub channel_id: Option<u64>,
    #[serde(default, rename = "channelName")]
    pub channel_name: Option<String>,
    #[serde(default)]
    pub pair: Option<Atom>,
    pub subscription: SubscriptionStatusPayload,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionStatusPayload {
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

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_decode_subscription_status() {
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
    fn test_decode_subscription_status_error() {
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
