use serde::Deserialize;
use serde::Serialize;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "auctionfeed")]
    Auction,
}

#[derive(Debug, Hash)]
pub struct Channel {
    pub ty: ChannelType,
    pub product_id: Atom,
}

impl<A> From<(ChannelType, A)> for Channel
where
    A: Into<Atom>,
{
    fn from((ty, pair): (ChannelType, A)) -> Self {
        Channel {
            ty,
            product_id: pair.into(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Subscribe {
    pub product_ids: Vec<Atom>,
    pub channels: Vec<ChannelType>,
}

#[derive(Debug, Deserialize)]
pub struct SubscribeResponse {
    pub channels: Vec<ChannelType>,
}

impl From<Channel> for Subscribe {
    fn from(channel: Channel) -> Self {
        let Channel { ty, product_id } = channel;
        Self {
            product_ids: vec![product_id],
            channels: vec![ty],
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Unsubscribe {
    pub product_ids: Vec<Atom>,
    pub channels: Vec<ChannelType>,
}
