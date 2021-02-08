use std::collections::BTreeMap;

use rust_decimal::Decimal;

use crate::{DiffOrderBookEvent, OrderBook};
use rust_decimal::prelude::Zero;

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

    pub fn push_diff(&mut self, update: DiffOrderBookEvent) -> Result<(), ()> {
        match self {
            OrderBookUpdater::Preparing { buffer } => buffer.push(update),
            OrderBookUpdater::Ready { state } => state.update(update)?,
        }
        Ok(())
    }

    pub fn init(&mut self, snapshot: OrderBook) -> Result<(), ()> {
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

    pub fn update(&mut self, diff: DiffOrderBookEvent) -> Result<(), ()> {
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
            if diff.final_update_id <= next_id {
                // Ignore an old update.
                return Ok(());
            }
            if diff.first_update_id > next_id {
                Err(())?
            }
            // ^^ ensures diff.first_update_id <= next_id && diff.final_update_id > next_id
            self.dirty = false;
        } else {
            if diff.first_update_id != next_id {
                Err(())?
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
