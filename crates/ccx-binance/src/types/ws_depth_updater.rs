use std::collections::BTreeMap;

use ccx_lib::order_book::PriceAndAmount;
use rust_decimal::Decimal;

use crate::api::spot::OrderBook;
use crate::api::spot::OrderBookRow;
use crate::types::ws_events::DepthUpdateEvent;

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum OrderBookSyncError {
    #[display("Order book is missing updates")]
    MissingUpdate,
}

pub struct OrderBookSync {
    last_update_id: i64,
    bids: BTreeMap<Decimal, Decimal>,
    asks: BTreeMap<Decimal, Decimal>,
}

impl OrderBookSync {
    pub fn new(
        OrderBook {
            last_update_id,
            bids,
            asks,
        }: OrderBook,
    ) -> Self {
        Self {
            last_update_id,
            bids: bids.into_iter().map(OrderBookRow::into_tuple).collect(),
            asks: asks.into_iter().map(OrderBookRow::into_tuple).collect(),
        }
    }

    pub fn update(&mut self, update_event: &DepthUpdateEvent) -> Result<(), OrderBookSyncError> {
        if update_event.last_update_id < self.last_update_id {
            // Ignore the event
            return Ok(());
        }
        if update_event.first_update_id > self.last_update_id + 1 {
            return Err(OrderBookSyncError::MissingUpdate);
        }
        update_book(&mut self.bids, &update_event.bids);
        update_book(&mut self.asks, &update_event.asks);
        self.last_update_id = update_event.last_update_id;

        Ok(())
    }

    pub fn last_update_id(&self) -> i64 {
        self.last_update_id
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

fn update_book(book: &mut BTreeMap<Decimal, Decimal>, updates: &[OrderBookRow]) {
    for offer in updates {
        if offer.qty.is_zero() {
            book.remove(&offer.price);
        } else {
            book.insert(offer.price, offer.qty);
        }
    }
}
