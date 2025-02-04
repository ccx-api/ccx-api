use serde::Deserialize;
use serde::Serialize;

use crate::util::OrderBook;
use crate::util::OrderLevel;
use crate::Atom;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderBookSnap {
    pub channel_id: u64,
    pub payload: OrderBookSnapPayload,
    pub channel_name: String,
    pub pair: Atom,
}

impl From<OrderBookSnap> for OrderBook {
    fn from(val: OrderBookSnap) -> Self {
        OrderBook {
            bids: val.payload.bids,
            asks: val.payload.asks,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderBookSnapPayload {
    #[serde(rename = "as")]
    pub asks: Vec<OrderLevel>,
    #[serde(rename = "bs")]
    pub bids: Vec<OrderLevel>,
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::super::*;

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
            UpstreamWebsocketMessage::Event(WsEvent::OrderBookSnap(e)) => {
                assert_eq!(e.channel_id, 2304);
                assert_eq!(e.payload.asks.len(), 2);
                assert_eq!(
                    e.payload.asks[0].price,
                    Decimal::from_str_exact("1226.20000").unwrap()
                );
                assert_eq!(e.payload.bids.len(), 2);
                assert_eq!(
                    e.payload.bids[0].price,
                    Decimal::from_str_exact("1225.20000").unwrap()
                );
            }
            _ => unreachable!(),
        }
    }
}
