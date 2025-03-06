use std::collections::BTreeMap;

use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use crate::api::spot::OrderBook;
use crate::api::spot::OrderBookRow;
use crate::types::ws_events::DepthUpdateEvent;

pub struct OrderBookSync {
    last_update_id: i64,
    bids: BTreeMap<Decimal, Decimal>,
    asks: BTreeMap<Decimal, Decimal>,
}

pub struct Fill {
    pub base_value: Decimal,
    pub quote_value: Decimal,
    pub exhausted: bool,
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

    pub fn update(&mut self, update_event: &DepthUpdateEvent) {
        if update_event.last_update_id < self.last_update_id {
            // Ignore the event
            return;
        }
        if update_event.first_update_id > self.last_update_id + 1 {
            panic!("Order book is missing updates");
        }
        update_book(&mut self.bids, &update_event.bids);
        update_book(&mut self.asks, &update_event.asks);
        self.last_update_id = update_event.last_update_id;
    }

    pub fn last_update_id(&self) -> i64 {
        self.last_update_id
    }

    pub fn bids(&self) -> &BTreeMap<Decimal, Decimal> {
        &self.bids
    }

    pub fn asks(&self) -> &BTreeMap<Decimal, Decimal> {
        &self.asks
    }

    pub fn next_bid(&self) -> Option<(&Decimal, &Decimal)> {
        self.bids.iter().rev().next()
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

    pub fn bid_base_depth(&self, base_qty_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for (&price, &qty) in self.bids.iter().rev() {
            let diff = base_qty_limit - base_value;
            let amount = diff.min(qty);
            base_value += amount;
            quote_value += amount * price;
            if amount == diff {
                exhausted = false;
                break;
            }
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    pub fn ask_base_depth(&self, base_qty_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for (&price, &qty) in self.asks.iter() {
            let diff = base_qty_limit - base_value;
            let amount = diff.min(qty);
            base_value += amount;
            quote_value += amount * price;
            if amount == diff {
                exhausted = false;
                break;
            }
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    pub fn spread(&self) -> Option<Decimal> {
        let ask = self.next_ask()?.0;
        let bid = self.next_bid()?.0;
        Some(ask - bid)
    }
}

impl Fill {
    pub fn price(&self) -> Decimal {
        self.quote_value / self.base_value
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
