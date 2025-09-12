use serde::Deserialize;
use serde::Serialize;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "auctionfeed")]
    Auction,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Hash, Serialize)]
pub struct Channel {
    pub name: ChannelType,
    pub product_ids: Vec<Atom>,
}

impl<A> From<(ChannelType, A)> for Channel
where
    A: Into<Atom>,
{
    fn from((ty, pair): (ChannelType, A)) -> Self {
        Channel {
            name: ty,
            product_ids: vec![pair.into()],
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Subscribe {
    pub product_ids: Vec<Atom>,
    pub channels: Vec<ChannelType>,
}

#[derive(Debug, Deserialize)]
pub struct SubscribeResponse {
    pub channels: Vec<Channel>,
}

impl From<Channel> for Subscribe {
    fn from(channel: Channel) -> Self {
        Self {
            product_ids: channel.product_ids,
            channels: vec![channel.name],
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Unsubscribe {
    pub product_ids: Vec<Atom>,
    pub channels: Vec<ChannelType>,
}
