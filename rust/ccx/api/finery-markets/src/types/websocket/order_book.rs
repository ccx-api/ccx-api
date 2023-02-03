use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

use crate::client::WebSocket;
use crate::types::BookItem;
use crate::types::Price;
use crate::types::Size;
use crate::LibResult;

use super::ActionKind;
use super::FeedId;
use super::FeedRequest;
use super::Pair;
use super::WsActionBookLevel;
use super::WsBookLevel;
use super::WsBookUpdate;
use super::WsInstrument;

#[derive(Clone, Debug)]
pub struct OrderMetaRate {
    pub ask: Option<BookItem>,
    pub bid: Option<BookItem>,
    pub last_updated: Instant,
}

impl Default for OrderMetaRate {
    fn default() -> Self {
        Self {
            ask: None,
            bid: None,
            last_updated: Instant::now(),
        }
    }
}

impl OrderMetaRate {
    pub fn is_valid(&self, duration: Duration) -> bool {
        if self.ask.is_none() && self.bid.is_none() {
            return false;
        }
        let now = Instant::now();
        if now.duration_since(self.last_updated) > duration {
            return false;
        }
        true
    }
}

#[derive(Clone)]
pub struct OrderBook {
    supports: HashSet<Pair>,
    feeds: HashMap<Pair, u64>,
    map: HashMap<u64, PairOrderBook>,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new(HashSet::new())
    }
}

impl OrderBook {
    pub fn with_supports(supports: impl IntoIterator<Item = Pair>) -> Self {
        Self::new(supports.into_iter().collect())
    }

    fn new(supports: HashSet<Pair>) -> Self {
        Self {
            supports,
            feeds: HashMap::new(),
            map: HashMap::new(),
        }
    }

    pub fn instruments(&self) -> Vec<Pair> {
        self.feeds.keys().map(|item| item.clone()).collect()
    }

    // pub fn order_meta(&self) -> HashMap<Pair, OrderMetaRate> {
    //     self.feeds.iter()
    //         .map(|(pair, id)| (pair, self.map.get(id)))
    //         .filter(|(_, book)| book.is_some())
    //         .map(|(pair, book)| {
    //             let rate = match book {
    //                 Some(book) => book.rate(),
    //                 None => OrderMetaRate::default(),
    //             };
    //             (pair.clone(), rate)
    //         }).collect()
    // }

    pub fn order_meta(&self, pair: &Pair) -> Option<OrderMetaRate> {
        let id = self.feeds.get(pair)?;
        let book = self.map.get(id)?;
        Some(book.rate())
    }

    fn get_id(&self, feed_id: impl Into<FeedId>) -> Option<u64> {
        match feed_id.into() {
            FeedId::Currency(_) => None,
            FeedId::Instrument(id) => Some(id),
            FeedId::Pair(pair) => self.feeds.get(&pair).map(|id| *id),
        }
    }

    pub async fn set_instruments(
        &mut self,
        ws: WebSocket,
        instruments: Vec<WsInstrument>,
    ) -> LibResult<()> {
        self.feeds = instruments
            .into_iter()
            .filter(|instrument| {
                if self.supports.is_empty() {
                    true
                } else {
                    self.supports.contains(&instrument.name)
                }
            })
            .map(|instrument| (instrument.name, instrument.id))
            .collect();
        for (_, id) in &self.feeds {
            self.map.insert(*id, PairOrderBook::new());
            subscribe_order_book(ws.clone(), *id).await?;
        }

        Ok(())
    }

    pub fn on_snapshot(&mut self, feed_id: FeedId, bids: Vec<WsBookLevel>, asks: Vec<WsBookLevel>) {
        let instrument_id = match self.get_id(feed_id) {
            Some(id) => id,
            None => return,
        };
        match self.map.get_mut(&instrument_id) {
            Some(book) => book.on_snapshot(bids, asks),
            None => {}
        }
    }

    pub fn on_update(&mut self, feed_id: FeedId, bids: Vec<WsBookUpdate>, asks: Vec<WsBookUpdate>) {
        let instrument_id = match self.get_id(feed_id) {
            Some(id) => id,
            None => return,
        };
        match self.map.get_mut(&instrument_id) {
            Some(book) => book.on_update(bids, asks),
            None => {}
        }
    }

    pub async fn update_instrument(
        &mut self,
        ws: WebSocket,
        action: ActionKind,
        ws_instrument: WsInstrument,
    ) -> LibResult<()> {
        let contains = if self.supports.is_empty() {
            true
        } else {
            self.supports.contains(&ws_instrument.name)
        };
        match action {
            ActionKind::Added if contains => {
                self.feeds.insert(ws_instrument.name, ws_instrument.id);
                self.map.insert(ws_instrument.id, PairOrderBook::new());
                subscribe_order_book(ws, ws_instrument.id).await?;
            }
            ActionKind::Removed => {
                self.feeds.remove(&ws_instrument.name);
                self.map.remove(&ws_instrument.id);
                unsubscribe_order_book(ws, ws_instrument.id).await?;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn subscribe_instruments(&self, ws: WebSocket) -> LibResult<()> {
        subscribe_instruments(ws).await
    }

    pub async fn unsubscribe_instruments(&self, ws: WebSocket) -> LibResult<()> {
        unsubscribe_instruments(ws).await
    }

    fn book(&self, feed_id: impl Into<FeedId>) -> Option<&PairOrderBook> {
        let instrument_id = match self.get_id(feed_id) {
            Some(id) => id,
            None => return None,
        };
        self.map.get(&instrument_id)
    }

    pub fn current_ask(&self, feed_id: impl Into<FeedId>) -> Option<(Price, Size)> {
        let book = self.book(feed_id)?;
        book.current_ask()
    }

    pub fn current_bid(&self, feed_id: impl Into<FeedId>) -> Option<(Price, Size)> {
        let book = self.book(feed_id)?;
        book.current_bid()
    }

    pub(crate) fn print_book(&self, feed_id: impl Into<FeedId>) {
        let instrument_id = match self.get_id(feed_id) {
            Some(id) => id,
            None => return,
        };
        let book = match self.map.get(&instrument_id) {
            Some(book) => book,
            None => return,
        };
        log::debug!(
            "print_book :: {:?} - {:?} :: {:?} - {:?}",
            book.current_bid(),
            book.bid_size(),
            book.current_ask(),
            book.ask_size(),
        );
    }

    pub async fn unsubscribe_book(&mut self, ws: WebSocket) -> LibResult<()> {
        for (_pair, instrument_id) in &self.feeds {
            unsubscribe_order_book(ws.clone(), *instrument_id).await?;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.feeds.clear();
        self.map.clear();
    }

    pub fn build_new(self) -> Self {
        Self {
            supports: self.supports,
            feeds: HashMap::new(),
            map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct PairOrderBook {
    last_update: Instant,
    bids: Levels,
    asks: Levels,
}

#[derive(Debug, Clone, Copy)]
enum OrderKind {
    Ask,
    Bid,
}

impl PairOrderBook {
    fn new() -> Self {
        Self {
            last_update: Instant::now(),
            bids: Levels::new(OrderKind::Bid),
            asks: Levels::new(OrderKind::Ask),
        }
    }

    fn rate(&self) -> OrderMetaRate {
        OrderMetaRate {
            ask: self
                .current_ask()
                .map(|(price, total_size)| BookItem { price, total_size }),
            bid: self
                .current_bid()
                .map(|(price, total_size)| BookItem { price, total_size }),
            last_updated: self.last_update,
        }
    }

    fn current_bid(&self) -> Option<(Price, Size)> {
        self.bids.current_price()
    }

    fn bid_size(&self) -> usize {
        self.bids.size()
    }

    fn current_ask(&self) -> Option<(Price, Size)> {
        self.asks.current_price()
    }

    fn ask_size(&self) -> usize {
        self.asks.size()
    }

    fn update_dt(&mut self) {
        self.last_update = Instant::now();
    }

    fn on_snapshot(&mut self, bids: Vec<WsBookLevel>, asks: Vec<WsBookLevel>) {
        // log::debug!("on_snapshot :: {:?} :: {:?}", bids, asks);
        self.update_dt();
        self.bids.on_snapshot(bids);
        self.asks.on_snapshot(asks);
    }

    fn on_update(&mut self, bids: Vec<WsBookUpdate>, asks: Vec<WsBookUpdate>) {
        // log::debug!("on_update :: {:?} :: {:?}", bids, asks);
        self.update_dt();
        self.bids.on_update(bids);
        self.asks.on_update(asks);
    }
}

#[derive(Debug, Clone)]
struct Levels {
    order_kind: OrderKind,
    map: BTreeMap<Price, Size>,
}

impl Levels {
    pub fn new(order_kind: OrderKind) -> Self {
        Self {
            order_kind,
            map: BTreeMap::new(),
        }
    }

    fn current_price(&self) -> Option<(Price, Size)> {
        match self.order_kind {
            OrderKind::Ask => self.first(),
            OrderKind::Bid => self.last(),
        }
    }

    fn first(&self) -> Option<(Price, Size)> {
        self.map.iter().next().map(|(k, v)| (*k, *v))
    }

    fn last(&self) -> Option<(Price, Size)> {
        self.map.iter().next_back().map(|(k, v)| (*k, *v))
    }

    fn size(&self) -> usize {
        self.map.len()
    }

    fn on_snapshot(&mut self, list: Vec<WsBookLevel>) {
        // log::debug!("on_snapshot :: {:?}", list);
        self.map = list
            .into_iter()
            .map(|item| (item.price, item.size))
            .collect();
    }

    fn on_update(&mut self, list: Vec<WsBookUpdate>) {
        // log::debug!("on_update :: {:?}", list);
        for item in list {
            match item.action {
                WsActionBookLevel::Added | WsActionBookLevel::Modified => self.on_modified(item),
                WsActionBookLevel::Removed => self.on_removed(item),
                WsActionBookLevel::RemovedSpecified => self.on_removed_specified(item),
            }
        }
    }

    fn on_modified(&mut self, item: WsBookUpdate) {
        // log::debug!("on_modified :: {:?} :: {:?}", self.order_kind, item);
        self.map.insert(item.price, item.size);
    }

    fn on_removed(&mut self, item: WsBookUpdate) {
        // log::debug!("on_removed :: {:?} :: {:?}", self.order_kind, item);
        self.map.remove(&item.price);
    }

    fn on_removed_specified(&mut self, item: WsBookUpdate) {
        // log::debug!(
        //     "on_removed_specified :: {:?} :: {:?}",
        //     self.order_kind,
        //     item
        // );
        match item.price {
            0 => self.map.clear(),
            price => {
                let target_key = self.current_price().map(|(k, _v)| k);
                match target_key {
                    Some(level_price) if level_price != price => {
                        self.map.remove(&level_price);
                    }
                    Some(_) => {
                        self.map.insert(price, item.size);
                    }
                    None => {}
                }
            }
        }
    }
}

async fn subscribe_instruments(ws: WebSocket) -> LibResult<()> {
    let feed_request = FeedRequest::Instruments;
    let _result = ws.subscribe(feed_request).await?;
    Ok(())
}

async fn unsubscribe_instruments(ws: WebSocket) -> LibResult<()> {
    let feed_request = FeedRequest::Instruments;
    let _result = ws.unsubscribe(feed_request).await?;
    Ok(())
}

async fn subscribe_order_book(ws: WebSocket, instrument_id: u64) -> LibResult<()> {
    let feed_request = FeedRequest::TradableOrderBooks;
    let feed_id = FeedId::from(instrument_id);
    let _result = ws.subscribe_feed(feed_request, feed_id).await?;
    Ok(())
}

async fn unsubscribe_order_book(ws: WebSocket, instrument_id: u64) -> LibResult<()> {
    let feed_request = FeedRequest::TradableOrderBooks;
    let feed_id = FeedId::from(instrument_id);
    let _result = ws.unsubscribe_feed(feed_request, feed_id).await?;
    Ok(())
}
