use std::collections::BTreeMap;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::ws_stream::OrderBookDiff;
use crate::KrakenResult;

#[derive(Debug)]
pub enum OrderBookUpdater {
    Preparing { buffer: Vec<OrderBookDiff> },
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderBook {
    pub bids: Vec<OrderLevel>,
    pub asks: Vec<OrderLevel>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderLevel {
    pub price: Decimal,
    pub qty: Decimal,
    pub timestamp: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
}

impl OrderLevel {
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

    pub fn push_diff(&mut self, update: OrderBookDiff) -> KrakenResult<()> {
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

    pub fn asks(&self) -> &BTreeMap<Decimal, Decimal> {
        &self.asks
    }

    pub fn bids(&self) -> &BTreeMap<Decimal, Decimal> {
        &self.bids
    }

    pub fn ask_low(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.iter().next()
    }

    pub fn bid_high(&self) -> Option<(&Decimal, &Decimal)> {
        self.bids.iter().last()
    }

    pub fn ask_avg(&self) -> Option<(Decimal, Decimal)> {
        // lowest 10 ask levels
        let levels = self.asks.iter().take(10);

        let mut total_price = Decimal::zero();
        let mut total_volume = Decimal::zero();
        let mut count = 0;

        for (price, volume) in levels {
            total_price += price * volume;
            total_volume += volume;
            count += 1;
        }

        if count == 0 || total_volume == Decimal::zero() {
            return None;
        }

        Some((
            total_price / total_volume,
            total_volume / Decimal::from(count),
        ))
    }

    pub fn bid_avg(&self) -> Option<(Decimal, Decimal)> {
        // highest 10 bid levels
        let levels = self.bids.iter().rev().take(10);

        let mut total_price = Decimal::zero();
        let mut total_volume = Decimal::zero();
        let mut count = 0;

        for (price, volume) in levels {
            total_price += price * volume;
            total_volume += volume;
            count += 1;
        }

        if count == 0 || total_volume == Decimal::zero() {
            return None;
        }

        Some((
            total_price / total_volume,
            total_volume / Decimal::from(count),
        ))
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

    pub fn spread(&self) -> Decimal {
        let ask = self.ask_low().map(|(p, _)| p).cloned().unwrap_or_default();
        let bid = self.bid_high().map(|(p, _)| p).cloned().unwrap_or_default();
        ask - bid
    }

    pub fn verify_checksum(&self) -> KrakenResult<()> {
        // TODO: need to implement checksum verification
        Ok(())
    }

    pub fn update(&mut self, diff: OrderBookDiff) -> KrakenResult<()> {
        if let Some(asks) = diff.asks {
            for ask_val in &asks.levels {
                if ask_val.qty.is_zero() {
                    log::trace!(" - removing ask: {:?}", ask_val.price);
                    self.asks.remove(&ask_val.price);
                } else {
                    log::trace!(" - inserting ask: {:?}", ask_val.price);
                    self.asks.insert(ask_val.price, ask_val.qty);
                }
            }
        }

        if let Some(bids) = diff.bids {
            for bid_val in &bids.levels {
                if bid_val.qty.is_zero() {
                    log::trace!(" - removing bid: {:?}", bid_val.price);
                    self.bids.remove(&bid_val.price);
                } else {
                    log::trace!(" - inserting bid: {:?}", bid_val.price);
                    self.bids.insert(bid_val.price, bid_val.qty);
                }
            }
        }

        self.verify_checksum()?;

        Ok(())
    }
}
