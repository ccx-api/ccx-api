use std::collections::BTreeMap;

use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{DiffOrderBookEvent, LibError, LibResult};

pub enum OrderBookUpdater {
    Preparing { buffer: Vec<DiffOrderBookEvent> },
    Ready { state: OrderBookState },
}

pub struct OrderBookState {
    last_update_id: u64,
    dirty: bool,
    asks: BTreeMap<Decimal, Decimal>,
    bids: BTreeMap<Decimal, Decimal>,
}

pub struct Fill {
    pub base_value: Decimal,
    pub quote_value: Decimal,
    pub exhausted: bool,
}

#[derive(Clone, Debug)]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Box<[Bid]>,
    pub asks: Box<[Ask]>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Bid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Ask {
    pub price: Decimal,
    pub qty: Decimal,
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

    pub fn push_diff(&mut self, update: DiffOrderBookEvent) -> LibResult<()> {
        match self {
            OrderBookUpdater::Preparing { buffer } => buffer.push(update),
            OrderBookUpdater::Ready { state } => state.update(update)?,
        }
        Ok(())
    }

    pub fn init(&mut self, snapshot: OrderBook) -> LibResult<()> {
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
                todo!()
            }
        }
    }
}

impl OrderBookState {
    pub fn new(snapshot: OrderBook) -> Self {
        OrderBookState {
            last_update_id: snapshot.last_update_id,
            dirty: true,
            asks: snapshot
                .asks
                .into_iter()
                .map(|v| (v.price, v.qty))
                .collect(),
            bids: snapshot
                .bids
                .into_iter()
                .map(|v| (v.price, v.qty))
                .collect(),
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

    pub fn next_ask(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.iter().next()
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

    pub fn update(&mut self, diff: DiffOrderBookEvent) -> LibResult<()> {
        /*
           Drop any event where final_update_id is <= lastUpdateId in the snapshot.

           The first processed event should have
               first_update_id <= lastUpdateId+1 AND final_update_id >= lastUpdateId+1.

           While listening to the stream, each new event's first_update_id should be equal
               to the previous event's final_update_id + 1.
        */
        let next_id = self.last_update_id + 1;
        log::trace!(
            "  next_id:  {},  last_update_id:  {},  first_update_id:  {},  final_update_id:  {}",
            next_id,
            self.last_update_id,
            diff.first_update_id,
            diff.final_update_id
        );

        if self.dirty {
            if diff.final_update_id < next_id {
                // Ignore an old update.
                return Ok(());
            }
            if diff.first_update_id > next_id {
                Err(LibError::other(format!(
                    "first_update_id > next_id:   {};   {}",
                    diff.first_update_id, next_id
                )))?
            }
            // ^^ ensures diff.first_update_id <= next_id && diff.final_update_id > next_id
            self.dirty = false;
        } else {
            if diff.first_update_id != next_id {
                Err(LibError::other(format!(
                    "first_update_id != next_id:   {};   {}",
                    diff.first_update_id, next_id
                )))?
            }
        }

        self.last_update_id = diff.final_update_id;

        for e in diff.asks {
            if e.qty.is_zero() {
                self.asks.remove(&e.price);
            } else {
                self.asks.insert(e.price, e.qty);
            }
        }
        for e in diff.bids {
            if e.qty.is_zero() {
                self.bids.remove(&e.price);
            } else {
                self.bids.insert(e.price, e.qty);
            }
        }
        Ok(())
    }
}
