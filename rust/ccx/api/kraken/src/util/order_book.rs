use std::collections::BTreeMap;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{DiffOrderBookEvent, KrakenResult};

#[derive(Debug)]
pub enum OrderBookUpdater {
    Preparing { buffer: Vec<DiffOrderBookEvent> },
    Ready { state: OrderBookState },
}

#[derive(Debug)]
pub struct OrderBookState {
    asks: BTreeMap<Decimal, Decimal>,
    bids: BTreeMap<Decimal, Decimal>,
}

#[derive(Debug)]
pub struct Fill {
    pub base_value: Decimal,
    pub quote_value: Decimal,
    pub exhausted: bool,
}

#[derive(Clone, Debug)]
pub struct OrderBook {
    pub bids: Box<[Bid]>,
    pub asks: Box<[Ask]>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct Bid {
    pub price: Decimal,
    pub qty: Decimal,
    pub timestamp: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
}

impl Bid {
    pub fn is_republished(&self) -> bool {
        self.update_type.as_deref() == Some("r")
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct Ask {
    pub price: Decimal,
    pub qty: Decimal,
    pub timestamp: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
}

impl Ask {
    pub fn is_republished(&self) -> bool {
        self.update_type.as_deref() == Some("r")
    }
}

impl OrderBookUpdater {
    pub fn new() -> Self {
        OrderBookUpdater::Preparing { buffer: vec![] }
    }

    pub fn state(&self) -> Option<&OrderBookState> {
        match self {
            OrderBookUpdater::Preparing { .. } => None,
            OrderBookUpdater::Ready { state } => Some(state),
        }
    }

    pub fn push_diff(&mut self, update: DiffOrderBookEvent) -> KrakenResult<()> {
        match self {
            OrderBookUpdater::Preparing { buffer } => buffer.push(update),
            OrderBookUpdater::Ready { state } => state.update(update)?,
        }
        Ok(())
    }

    pub fn init(&mut self, snapshot: OrderBook) -> KrakenResult<()> {
        match self {
            OrderBookUpdater::Preparing { buffer } => {
                let mut state = OrderBookState::new(snapshot);
                for diff in buffer.drain(..) {
                    state.update(diff)?;
                }
                *self = OrderBookUpdater::Ready { state };
                Ok(())
            }
            OrderBookUpdater::Ready { .. } => {
                log::warn!("OrderBookUpdater already initialized");
                Ok(())
            }
        }
    }
}

impl Default for OrderBookUpdater {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBookState {
    pub fn new(snapshot: OrderBook) -> Self {
        OrderBookState {
            asks: snapshot.asks.iter().map(|v| (v.price, v.qty)).collect(),
            bids: snapshot.bids.iter().map(|v| (v.price, v.qty)).collect(),
        }
    }

    pub fn bids(&self) -> &BTreeMap<Decimal, Decimal> {
        &self.bids
    }

    pub fn asks(&self) -> &BTreeMap<Decimal, Decimal> {
        &self.asks
    }

    pub fn next_bid(&self) -> Option<(&Decimal, &Decimal)> {
        self.bids.iter().next_back()
    }

    pub fn next_bid_price(&self) -> Option<Decimal> {
        self.bids.iter().next_back().map(|(price, _qty)| *price)
    }

    pub fn next_ask(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.iter().next()
    }

    pub fn next_ask_price(&self) -> Option<Decimal> {
        self.asks.iter().next().map(|(price, _qty)| *price)
    }

    pub fn bid_volume(&self, price_limit: &Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for (price, volume) in self.bids.iter().rev() {
            if price_limit < price {
                exhausted = false;
                break;
            }
            base_value += volume;
            quote_value += volume * price;
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    pub fn ask_volume(&self, price_limit: &Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for (price, volume) in self.asks.iter() {
            if price_limit > price {
                exhausted = false;
                break;
            }
            base_value += volume;
            quote_value += volume * price;
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    pub fn update(&mut self, diff: DiffOrderBookEvent) -> KrakenResult<()> {
        if let Some(ask_data) = diff.ask_data {
            for e in &ask_data.values {
                if e.is_republished() {
                    continue;
                }
                if e.qty.is_zero() {
                    self.asks.remove(&e.price);
                } else {
                    self.asks.insert(e.price, e.qty);
                }
            }
        }

        if let Some(bid_data) = diff.bid_data {
            for e in &bid_data.values {
                if e.is_republished() {
                    continue;
                }
                if e.qty.is_zero() {
                    self.bids.remove(&e.price);
                } else {
                    self.bids.insert(e.price, e.qty);
                }
            }
        }
        Ok(())
    }
}
