use ccx_api_lib::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::Atom;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Trade {
    pub channel_id: u64,
    pub data: Vec<TradePayload>,
    pub channel_name: String,
    pub pair: Atom,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TradePayload {
    #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
    pub price: Decimal,
    #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
    pub volume: Decimal,
    #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
    pub time: Decimal,
    pub side: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    pub misc: String,
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::super::*;

    #[test]
    fn test_decode_trade_event() {
        let input = r#"[
            0,
            [
                ["5541.20000","0.15850568","1534614057.321597","s","l",""],
                ["6060.00000","0.02455000","1534614057.324998","b","l",""]
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
}
