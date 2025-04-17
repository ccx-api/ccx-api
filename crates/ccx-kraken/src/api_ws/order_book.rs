use std::collections::BTreeMap;

use bon::Builder;
use ccx_lib::order_book::PriceAndAmount;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use smallvec::SmallVec;

use crate::api::spot::market::AssetDepthInfo;
use crate::types::currency_pair::CurrencyPair;

/// Order book level
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Depth {
    L10 = 10,
    L25 = 25,
    L100 = 100,
    L500 = 500,
    L1000 = 1000,
}

/// Order book WebSocket request payload
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(on(CurrencyPair, into))]
pub struct OrderBookChannel {
    symbol: Vec<CurrencyPair>,
    #[builder(default = Depth::L10)]
    depth: Depth,
    #[builder(default = true)]
    snapshot: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OrderBookChannelSubscribed {
    pub symbol: CurrencyPair,
    pub depth: Depth,
    pub snapshot: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PriceAndQty {
    pub price: Decimal,
    pub qty: Decimal,
}

impl PriceAndQty {
    pub fn into_tuple(self) -> (Decimal, Decimal) {
        (self.price, self.qty)
    }
}

impl From<PriceAndQty> for PriceAndAmount {
    fn from(value: PriceAndQty) -> Self {
        Self {
            price: value.price,
            amount: value.qty,
        }
    }
}

/// Represents a snapshot of the order book.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OrderBookSnapshot {
    /// Currency pair.
    pub symbol: CurrencyPair,

    /// Checksum of the top 10 bids and asks
    pub checksum: i64,

    /// Top level bids in the current snapshot, sorted by price from high to low.
    pub asks: SmallVec<[PriceAndQty; 1]>,

    /// Top level asks in the current snapshot, sorted by price from low to high.
    pub bids: SmallVec<[PriceAndQty; 1]>,
}

/// Represents an update of the order book.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OrderBookUpdate {
    /// Currency pair.
    pub symbol: CurrencyPair,

    /// Checksum of the top 10 bids and asks
    pub checksum: i64,

    /// Timestamp of the update
    pub timestamp: String,

    /// Top level bids in the current snapshot, sorted by price from high to low.
    pub asks: SmallVec<[PriceAndQty; 1]>,

    /// Top level asks in the current snapshot, sorted by price from low to high.
    pub bids: SmallVec<[PriceAndQty; 1]>,
}

#[derive(Default)]
pub struct OrderBookSync {
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Decimal, Decimal>,
}

impl OrderBookSync {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_depth_response(asset_depth: AssetDepthInfo) -> Self {
        Self {
            bids: asset_depth
                .bids
                .into_iter()
                .map(|info| (info.price, info.volume))
                .collect(),
            asks: asset_depth
                .asks
                .into_iter()
                .map(|info| (info.price, info.volume))
                .collect(),
        }
    }

    pub fn set_from_snapshot(&mut self, OrderBookSnapshot { bids, asks, .. }: OrderBookSnapshot) {
        *self = Self {
            bids: bids.into_iter().map(PriceAndQty::into_tuple).collect(),
            asks: asks.into_iter().map(PriceAndQty::into_tuple).collect(),
        }
    }

    pub fn update(&mut self, update_event: &OrderBookUpdate) {
        update_book(&mut self.bids, &update_event.bids);
        update_book(&mut self.asks, &update_event.asks);
    }
}

impl ccx_lib::order_book::OrderBook for OrderBookSync {
    fn asks(&self) -> impl ExactSizeIterator<Item = PriceAndAmount> {
        self.asks.iter().map(PriceAndAmount::from)
    }

    fn bids(&self) -> impl ExactSizeIterator<Item = PriceAndAmount> {
        self.bids.iter().rev().map(PriceAndAmount::from)
    }
}

fn update_book(book: &mut BTreeMap<Decimal, Decimal>, updates: &[PriceAndQty]) {
    for offer in updates {
        if offer.qty.is_zero() {
            book.remove(&offer.price);
        } else {
            book.insert(offer.price, offer.qty);
        }
    }
}
