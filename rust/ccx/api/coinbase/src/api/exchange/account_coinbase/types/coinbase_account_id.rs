use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Atom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CoinbaseAccountId {
    Uuid(Uuid),
    Ticker(Atom),
}

impl Display for CoinbaseAccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoinbaseAccountId::Uuid(uuid) => write!(f, "{:?}", uuid),
            CoinbaseAccountId::Ticker(ticker) => write!(f, "{}", ticker),
        }
    }
}
