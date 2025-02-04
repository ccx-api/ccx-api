use serde::Deserialize;
use serde::Serialize;

use crate::util::OrderLevel;
use crate::Atom;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OrderBookDiff {
    pub channel_id: u64,
    pub asks: Option<OrderBookDiffAsk>,
    pub bids: Option<OrderBookDiffBid>,
    pub channel_name: String,
    pub pair: Atom,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderBookDiffAsk {
    #[serde(rename = "a", alias = "as")]
    pub levels: Vec<OrderLevel>,
    #[serde(default, rename = "c")]
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderBookDiffBid {
    #[serde(rename = "b", alias = "bs")]
    pub levels: Vec<OrderLevel>,
    #[serde(default, rename = "c")]
    pub checksum: Option<String>,
}

mod deser {
    use std::fmt;

    use serde::de::Deserialize;
    use serde::de::Deserializer;
    use serde::de::Error;
    use serde::de::SeqAccess;
    use serde::de::Visitor;

    use super::*;

    #[derive(Debug, serde::Deserialize, Clone)]
    #[serde(untagged)]
    enum DOBType {
        ChannelID(u64),
        Asks(OrderBookDiffAsk),
        Bids(OrderBookDiffBid),
        ChannelNameOrPair(String),
    }

    impl DOBType {
        pub fn as_channel_id(&self) -> Option<u64> {
            match self {
                DOBType::ChannelID(v) => Some(*v),
                _ => None,
            }
        }

        pub fn as_asks(&self) -> Option<OrderBookDiffAsk> {
            match self {
                DOBType::Asks(v) => Some(v.clone()),
                _ => None,
            }
        }

        pub fn as_bids(&self) -> Option<OrderBookDiffBid> {
            match self {
                DOBType::Bids(v) => Some(v.clone()),
                _ => None,
            }
        }

        pub fn as_channel_name(&self) -> Option<String> {
            match self {
                DOBType::ChannelNameOrPair(v) => Some(v.clone()),
                _ => None,
            }
        }

        pub fn as_pair(&self) -> Option<Atom> {
            match self {
                DOBType::ChannelNameOrPair(v) => Some(v.clone().into()),
                _ => None,
            }
        }
    }

    struct OrderBookDiffVisitor {}

    impl<'de> Visitor<'de> for OrderBookDiffVisitor {
        type Value = OrderBookDiff;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "[u64, Option<DiffOrderBookAskData>, \
                Option<DiffOrderBookBidData>, String, Atom]",
            )
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
            let mut values = Vec::new();
            while let Some(dob) = access.next_element::<DOBType>()? {
                values.push(dob);
            }

            let err_channel_id = Error::custom("bad `channel_id` (0)");
            let err_ask = Error::custom("bad `ask` data (1-2)");
            let err_bid = Error::custom("bad `bid` data (1-2)");
            let err_ask_or_bid = Error::custom("bad `ask` or `bid` data (1)");
            let err_name = Error::custom("bad `name` (1-3)");
            let err_pair = Error::custom("bad `pair` (2-4)");

            let (channel_id, asks, bids, channel_name, pair) = match &values[..] {
                [id, ask, bid, name, pair] => (
                    id.as_channel_id().ok_or(err_channel_id)?,
                    Some(ask.as_asks().ok_or(err_ask)?),
                    Some(bid.as_bids().ok_or(err_bid)?),
                    name.as_channel_name().ok_or(err_name)?,
                    pair.as_pair().ok_or(err_pair)?,
                ),
                [id, ask_or_bid, name, pair] => {
                    let (ask, bid) = match ask_or_bid {
                        DOBType::Asks(v) => (Some(v.clone()), None),
                        DOBType::Bids(v) => (None, Some(v.clone())),
                        _ => Err(err_ask_or_bid)?,
                    };

                    (
                        id.as_channel_id().ok_or(err_channel_id)?,
                        ask,
                        bid,
                        name.as_channel_name().ok_or(err_name)?,
                        pair.as_pair().ok_or(err_pair)?,
                    )
                }
                [id, name, pair] => (
                    id.as_channel_id().ok_or(err_channel_id)?,
                    None,
                    None,
                    name.as_channel_name().ok_or(err_name)?,
                    pair.as_pair().ok_or(err_pair)?,
                ),
                _ => Err(Error::duplicate_field("wrong format"))?,
            };

            Ok(OrderBookDiff {
                channel_id,
                asks,
                bids,
                channel_name,
                pair,
            })
        }
    }

    impl<'de> Deserialize<'de> for OrderBookDiff {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(OrderBookDiffVisitor {})
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::super::*;

    #[test]
    fn test_decode_diff_order_book_event_ask() {
        let input = r#"[
            2240,
            {"a":[["0.96400000","6031.62994854","1657008774.509615"]]},
            "book-100",
            "USDT/EUR"
        ]"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::OrderBookDiff(e)) => {
                assert_eq!(e.channel_id, 2240);

                if let Some(ask) = e.asks {
                    assert_eq!(ask.levels.len(), 1);
                    assert_eq!(
                        ask.levels[0].price,
                        Decimal::from_str_exact("0.96400000").unwrap()
                    );
                } else {
                    unreachable!();
                }

                assert_eq!(e.bids, None);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_diff_order_book_event_bid() {
        let input = r#"[
            2240,
            {
                "b":[
                    ["0.96400000","0.00000000","1657008774.509730"],
                    ["0.94440000","1642.43686753","1657008552.389847","r"]
                ],
                "c":"1208501689"
            },
            "book-100",
            "USDT/EUR"
        ]"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::OrderBookDiff(e)) => {
                assert_eq!(e.channel_id, 2240);

                if let Some(bids) = e.bids {
                    assert_eq!(bids.levels.len(), 2);
                    assert_eq!(
                        bids.levels[0].price,
                        Decimal::from_str_exact("0.96400000").unwrap()
                    );
                } else {
                    unreachable!();
                }

                assert_eq!(e.asks, None);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_decode_diff_order_book_event_multi() {
        let input = r#"[
            2240,
            {"a":[["0.96400000","6031.62994854","1657008774.509615"]]},
            {
                "b":[
                    ["0.96400000","0.00000000","1657008774.509730"],
                    ["0.94440000","1642.43686753","1657008552.389847","r"]
                ],
                "c":"1208501689"
            },
            "book-100",
            "USDT/EUR"
        ]"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::OrderBookDiff(e)) => {
                assert_eq!(e.channel_id, 2240);

                if let Some(asks) = e.asks {
                    assert_eq!(asks.levels.len(), 1);
                    assert_eq!(
                        asks.levels[0].price,
                        Decimal::from_str_exact("0.96400000").unwrap()
                    );
                } else {
                    unreachable!();
                }

                if let Some(bids) = e.bids {
                    assert_eq!(bids.levels.len(), 2);
                    assert_eq!(
                        bids.levels[0].price,
                        Decimal::from_str_exact("0.96400000").unwrap()
                    );
                } else {
                    unreachable!();
                }
            }
            _ => unreachable!(),
        }
    }
}
