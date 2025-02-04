use std::borrow::Cow;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use crate::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CoinbaseAccountId {
    Uuid(Uuid),
    Ticker(Cow<'static, str>),
}

impl Display for CoinbaseAccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoinbaseAccountId::Uuid(uuid) => write!(f, "{:?}", uuid),
            CoinbaseAccountId::Ticker(ticker) => write!(f, "{}", ticker),
        }
    }
}
