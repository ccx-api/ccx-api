use serde::{Deserialize, Serialize};

use crate::util::{Ask, Bid};
use crate::Atom;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDiffEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
}
