use bytes::Bytes;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::CancelReason;
use crate::types::ClientId;
use crate::types::ClientOrderId;
use crate::types::DealId;
use crate::types::Flags;
use crate::types::OrderId;
use crate::types::Pair;
use crate::types::Price;
use crate::types::SideByRepr;
use crate::types::Size;
use crate::types::Timestamp;
use crate::LibError;
use crate::LibResult;

mod order_book;
mod positions;
mod transactions;

pub use order_book::OrderBook;
pub use positions::Positions;
pub use transactions::Transactions;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "bind")]
    Bind,
    #[serde(rename = "unbind")]
    Unbind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum FeedRequest {
    #[serde(rename = "I")]
    Instruments,
    #[serde(rename = "P")]
    PositionOrders,
    #[serde(rename = "G")]
    GlobalLimits,
    #[serde(rename = "L")]
    CounterpartyLimits,
    #[serde(rename = "M")]
    CounterpartyMutualLimits,
    #[serde(rename = "B")]
    GlobalOrderBooks,
    #[serde(rename = "F")]
    TradableOrderBooks,
    #[serde(rename = "R")]
    SettlementRequests,
    #[serde(rename = "N")]
    SettlementTransactions,
    #[serde(rename = "K")]
    PositionFeed,
    #[serde(rename = "O")]
    Orders,
    #[serde(rename = "S")]
    SettlementOrders,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum FeedId {
    Instrument(u64),
    Pair(Pair),
    Currency(String),
}

impl From<u64> for FeedId {
    fn from(value: u64) -> Self {
        Self::Instrument(value)
    }
}

impl From<Pair> for FeedId {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl From<String> for FeedId {
    fn from(value: String) -> Self {
        Self::Currency(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, actix::Message)]
#[rtype(result = "crate::error::LibResult<()>")]
pub struct WsRequest {
    event: Event,
    feed: FeedRequest,
    #[serde(rename = "feedId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    feed_id: Option<FeedId>,
}

impl WsRequest {
    fn new(event: Event, feed: FeedRequest, feed_id: Option<FeedId>) -> Self {
        Self {
            event,
            feed,
            feed_id,
        }
    }

    pub fn subscribe(feed: FeedRequest) -> Self {
        Self::new(Event::Bind, feed, None)
    }

    pub fn unsubscribe(feed: FeedRequest) -> Self {
        Self::new(Event::Unbind, feed, None)
    }

    pub fn subscribe_feed(feed: FeedRequest, feed_id: FeedId) -> Self {
        Self::new(Event::Bind, feed, Some(feed_id))
    }

    pub fn unsubscribe_feed(feed: FeedRequest, feed_id: FeedId) -> Self {
        Self::new(Event::Unbind, feed, Some(feed_id))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum FeedResponse {
    #[serde(rename = "I")]
    Instrument,
    #[serde(rename = "C")]
    Currency,
    #[serde(rename = "P")]
    PositionOrder,
    #[serde(rename = "O")]
    Order,
    #[serde(rename = "S")]
    Settlement,
    #[serde(rename = "G")]
    GlobalLimit,
    #[serde(rename = "L")]
    CounterpartyLimit,
    #[serde(rename = "M")]
    CounterpartyMutualLimit,
    #[serde(rename = "B")]
    BookLevel,
    #[serde(rename = "F")]
    TradeBookLevel,
    #[serde(rename = "R")]
    SettlementRequest,
    #[serde(rename = "N")]
    SettlementTransaction,
    #[serde(rename = "K")]
    PositionFeed,
    #[serde(rename = "X")]
    Connection,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActionKind {
    #[serde(rename = "S")]
    Snapshot,
    #[serde(rename = "+")]
    Added,
    #[serde(rename = "M")]
    Modified,
    #[serde(rename = "-")]
    Removed,
    #[serde(rename = "Z")]
    FailedSubscribe,
    #[serde(rename = "U")]
    Unsubscribed,
    #[serde(rename = "D")]
    Executed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsResponse {
    pub feed: FeedResponse,
    pub feed_id: FeedId,
    pub action: ActionKind,
    pub value: Value,
}

impl TryFrom<Bytes> for WsResponse {
    type Error = LibError;
    fn try_from(bytes: Bytes) -> LibResult<Self> {
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct InstrumentSnapshot {
    pub currencies: Vec<WsCurrency>,
    pub instruments: Vec<WsInstrument>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct PositionOrderSnapshot {
    #[serde(rename = "nextDealOrSettlementOrTransactionId")]
    pub next_deal_or_settlement_or_transaction_id: DealId,
    pub positions: Vec<WsPosition>,
    pub orders: Vec<WsOrder>,
    pub settlement_orders: Vec<WsSettlementOrder>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct OrderBookSnapshot {
    pub bids: Vec<WsBookLevel>,
    pub asks: Vec<WsBookLevel>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct OrderBookLevelUpdate {
    pub bids: Vec<WsBookUpdate>,
    pub asks: Vec<WsBookUpdate>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SettlementRequests {
    pub incoming_requests: Vec<WsSettlementRequest>,
    pub outgoing_requests: Vec<WsSettlementRequest>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SettlementTransactionSnapshot {
    #[serde(rename = "nextDealOrSettlementOrTransactionId")]
    pub next_deal_or_settlement_or_transaction_id: DealId,
    pub incoming_requests: Vec<WsSettlementTransaction>,
    pub outgoing_requests: Vec<WsSettlementTransaction>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SettlementTransactions {
    pub incoming_requests: Vec<WsSettlementTransaction>,
    pub outgoing_requests: Vec<WsSettlementTransaction>,
}

#[derive(Debug)]
pub enum WsResponseData {
    Code(u32),
    InstrumentSnapshot(InstrumentSnapshot),
    Currency(WsCurrency),
    Instrument(WsInstrument),
    PositionOrderSnapshot(PositionOrderSnapshot),
    Order(WsOrder),
    Deal(WsDeal),
    SettlementOrder(WsSettlementOrder),
    SettlementDeal(WsSettlementDeal),
    GlobalLimitSnapshot(Vec<WsGlobalLimit>),
    GlobalLimit(WsGlobalLimit),
    CounterpartyLimitSnapshot(Vec<WsCounterpartyLimit>),
    CounterpartyLimit(WsCounterpartyLimit),
    CounterpartyMutualLimitSnapshot(Vec<WsCounterpartyMutualLimit>),
    CounterpartyMutualLimit(WsCounterpartyMutualLimit),
    OrderBookSnapshot(OrderBookSnapshot),
    OrderBookLevelUpdate(OrderBookLevelUpdate),
    SettlementRequests(SettlementRequests),
    SettlementTransactionSnapshot(SettlementTransactionSnapshot),
    SettlementTransactions(SettlementTransactions),
    PositionFeedSnapshot(Vec<WsPosition>),
    Position(WsPosition),
    OrderSnapshot(Vec<WsOrder>),
    SettlementOrderSnapshot(Vec<WsSettlementOrder>),
}

impl WsResponse {
    pub fn connected(&self) -> LibResult<bool> {
        let data = match self.data()? {
            Some(data) => data,
            None => return Ok(false),
        };
        match (self.feed, data) {
            (FeedResponse::Connection, WsResponseData::Code(0)) => Ok(true),
            _ => Ok(false),
        }
    }

    fn data_<T: serde::de::DeserializeOwned>(&self) -> LibResult<T> {
        Ok(serde_json::from_value(self.value.clone())?)
    }

    pub fn data(&self) -> LibResult<Option<WsResponseData>> {
        let data = match (self.feed, self.action) {
            (_, ActionKind::FailedSubscribe) => {
                let value = self.data_::<u32>()?;
                Some(WsResponseData::Code(value))
            }
            (_, ActionKind::Unsubscribed) => {
                let value = self.data_::<u32>()?;
                Some(WsResponseData::Code(value))
            }

            (FeedResponse::Instrument, ActionKind::Snapshot) => {
                let value = self.data_::<InstrumentSnapshot>()?;
                Some(WsResponseData::InstrumentSnapshot(value))
            }
            (FeedResponse::Currency, ActionKind::Added) => {
                let value = self.data_::<WsCurrency>()?;
                Some(WsResponseData::Currency(value))
            }
            (FeedResponse::Currency, ActionKind::Modified) => {
                let value = self.data_::<WsCurrency>()?;
                Some(WsResponseData::Currency(value))
            }
            (FeedResponse::Currency, ActionKind::Removed) => {
                let value = self.data_::<WsCurrency>()?;
                Some(WsResponseData::Currency(value))
            }
            (FeedResponse::Instrument, ActionKind::Added) => {
                let value = self.data_::<WsInstrument>()?;
                Some(WsResponseData::Instrument(value))
            }
            (FeedResponse::Instrument, ActionKind::Modified) => {
                let value = self.data_::<WsInstrument>()?;
                Some(WsResponseData::Instrument(value))
            }
            (FeedResponse::Instrument, ActionKind::Removed) => {
                let value = self.data_::<WsInstrument>()?;
                Some(WsResponseData::Instrument(value))
            }

            (FeedResponse::PositionOrder, ActionKind::Snapshot) => {
                let value = self.data_::<PositionOrderSnapshot>()?;
                Some(WsResponseData::PositionOrderSnapshot(value))
            }
            (FeedResponse::Order, ActionKind::Added) => {
                let value = self.data_::<WsOrder>()?;
                Some(WsResponseData::Order(value))
            }
            (FeedResponse::Order, ActionKind::Executed) => {
                let value = self.data_::<WsDeal>()?;
                Some(WsResponseData::Deal(value))
            }
            (FeedResponse::Settlement, ActionKind::Added) => {
                let value = self.data_::<WsSettlementOrder>()?;
                Some(WsResponseData::SettlementOrder(value))
            }
            (FeedResponse::Settlement, ActionKind::Modified) => {
                let value = self.data_::<WsSettlementOrder>()?;
                Some(WsResponseData::SettlementOrder(value))
            }
            (FeedResponse::Settlement, ActionKind::Executed) => {
                let value = self.data_::<WsSettlementDeal>()?;
                Some(WsResponseData::SettlementDeal(value))
            }
            (FeedResponse::Settlement, ActionKind::Removed) => {
                let value = self.data_::<WsSettlementOrder>()?;
                Some(WsResponseData::SettlementOrder(value))
            }

            (FeedResponse::GlobalLimit, ActionKind::Snapshot) => {
                let value = self.data_::<Vec<WsGlobalLimit>>()?;
                Some(WsResponseData::GlobalLimitSnapshot(value))
            }
            (FeedResponse::GlobalLimit, ActionKind::Modified) => {
                let value = self.data_::<WsGlobalLimit>()?;
                Some(WsResponseData::GlobalLimit(value))
            }

            (FeedResponse::CounterpartyLimit, ActionKind::Snapshot) => {
                let value = self.data_::<Vec<WsCounterpartyLimit>>()?;
                Some(WsResponseData::CounterpartyLimitSnapshot(value))
            }
            (FeedResponse::CounterpartyLimit, ActionKind::Added) => {
                let value = self.data_::<WsCounterpartyLimit>()?;
                Some(WsResponseData::CounterpartyLimit(value))
            }
            (FeedResponse::CounterpartyLimit, ActionKind::Modified) => {
                let value = self.data_::<WsCounterpartyLimit>()?;
                Some(WsResponseData::CounterpartyLimit(value))
            }
            (FeedResponse::CounterpartyLimit, ActionKind::Removed) => {
                let value = self.data_::<WsCounterpartyLimit>()?;
                Some(WsResponseData::CounterpartyLimit(value))
            }

            (FeedResponse::CounterpartyMutualLimit, ActionKind::Snapshot) => {
                let value = self.data_::<Vec<WsCounterpartyMutualLimit>>()?;
                Some(WsResponseData::CounterpartyMutualLimitSnapshot(value))
            }
            (FeedResponse::CounterpartyMutualLimit, ActionKind::Added) => {
                let value = self.data_::<WsCounterpartyMutualLimit>()?;
                Some(WsResponseData::CounterpartyMutualLimit(value))
            }
            (FeedResponse::CounterpartyMutualLimit, ActionKind::Modified) => {
                let value = self.data_::<WsCounterpartyMutualLimit>()?;
                Some(WsResponseData::CounterpartyMutualLimit(value))
            }
            (FeedResponse::CounterpartyMutualLimit, ActionKind::Removed) => {
                let value = self.data_::<WsCounterpartyMutualLimit>()?;
                Some(WsResponseData::CounterpartyMutualLimit(value))
            }

            (FeedResponse::BookLevel, ActionKind::Snapshot) => {
                let value = self.data_::<OrderBookSnapshot>()?;
                Some(WsResponseData::OrderBookSnapshot(value))
            }
            (FeedResponse::BookLevel, ActionKind::Modified) => {
                let value = self.data_::<OrderBookLevelUpdate>()?;
                Some(WsResponseData::OrderBookLevelUpdate(value))
            }
            (FeedResponse::TradeBookLevel, ActionKind::Snapshot) => {
                let value = self.data_::<OrderBookSnapshot>()?;
                Some(WsResponseData::OrderBookSnapshot(value))
            }
            (FeedResponse::TradeBookLevel, ActionKind::Modified) => {
                let value = self.data_::<OrderBookLevelUpdate>()?;
                Some(WsResponseData::OrderBookLevelUpdate(value))
            }

            (FeedResponse::SettlementRequest, ActionKind::Snapshot) => {
                let value = self.data_::<SettlementRequests>()?;
                Some(WsResponseData::SettlementRequests(value))
            }
            (FeedResponse::SettlementRequest, ActionKind::Added) => {
                let value = self.data_::<SettlementRequests>()?;
                Some(WsResponseData::SettlementRequests(value))
            }
            (FeedResponse::SettlementRequest, ActionKind::Removed) => {
                let value = self.data_::<SettlementRequests>()?;
                Some(WsResponseData::SettlementRequests(value))
            }

            (FeedResponse::SettlementTransaction, ActionKind::Snapshot) => {
                let value = self.data_::<SettlementTransactionSnapshot>()?;
                Some(WsResponseData::SettlementTransactionSnapshot(value))
            }
            (FeedResponse::SettlementTransaction, ActionKind::Added) => {
                let value = self.data_::<SettlementTransactions>()?;
                Some(WsResponseData::SettlementTransactions(value))
            }
            (FeedResponse::SettlementTransaction, ActionKind::Modified) => {
                let value = self.data_::<SettlementTransactions>()?;
                Some(WsResponseData::SettlementTransactions(value))
            }
            (FeedResponse::SettlementTransaction, ActionKind::Executed) => {
                let value = self.data_::<SettlementTransactions>()?;
                Some(WsResponseData::SettlementTransactions(value))
            }
            (FeedResponse::SettlementTransaction, ActionKind::Removed) => {
                let value = self.data_::<SettlementTransactions>()?;
                Some(WsResponseData::SettlementTransactions(value))
            }

            (FeedResponse::PositionFeed, ActionKind::Snapshot) => {
                let value = self.data_::<Vec<WsPosition>>()?;
                Some(WsResponseData::PositionFeedSnapshot(value))
            }
            (FeedResponse::PositionFeed, ActionKind::Added) => {
                let value = self.data_::<WsPosition>()?;
                Some(WsResponseData::Position(value))
            }
            (FeedResponse::PositionFeed, ActionKind::Modified) => {
                let value = self.data_::<WsPosition>()?;
                Some(WsResponseData::Position(value))
            }
            (FeedResponse::PositionFeed, ActionKind::Removed) => {
                let value = self.data_::<WsPosition>()?;
                Some(WsResponseData::Position(value))
            }

            (FeedResponse::Order, ActionKind::Snapshot) => {
                let value = self.data_::<Vec<WsOrder>>()?;
                Some(WsResponseData::OrderSnapshot(value))
            }
            (FeedResponse::Order, ActionKind::Removed) => {
                let value = self.data_::<WsOrder>()?;
                Some(WsResponseData::Order(value))
            }

            (FeedResponse::Settlement, ActionKind::Snapshot) => {
                let value = self.data_::<Vec<WsSettlementOrder>>()?;
                Some(WsResponseData::SettlementOrderSnapshot(value))
            }
            _ => None,
        };
        Ok(data)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsCurrency {
    pub name: String,
    pub id: ClientId,
    #[serde(rename = "balanceStep")]
    pub balance_step: Size,
    #[serde(rename = "usdPrice")]
    pub usd_price: Price,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsInstrument {
    pub name: Pair,
    pub id: u64,
    #[serde(rename = "assetCurrency")]
    pub asset_currency: String,
    #[serde(rename = "balanceCurrency")]
    pub balance_currency: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsPosition {
    #[serde(rename = "currencyName")]
    pub currency_name: String,
    pub value: Size,
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsOrder {
    #[serde(rename = "instrumentName")]
    pub instrument_name: String,
    #[serde(rename = "orderType")]
    pub order_type: u16,
    #[serde(rename = "orderSide")]
    pub order_side: SideByRepr,
    #[serde(rename = "orderCancelReason")]
    pub order_cancel_reason: CancelReason,
    #[serde(rename = "orderId")]
    pub order_id: OrderId,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    #[serde(rename = "orderPrice")]
    pub order_price: Price,
    #[serde(rename = "initialSize")]
    pub initial_size: Size,
    #[serde(rename = "sizeLeft")]
    pub size_left: Size,
    #[serde(rename = "createdAt")]
    pub created_at: Timestamp,
    #[serde(rename = "bySizeOrByVolume")]
    pub by_size_or_by_volume: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsDeal {
    #[serde(rename = "instrumentName")]
    pub instrument_name: Pair,
    #[serde(rename = "orderType")]
    pub order_type: u16,
    #[serde(rename = "orderSide")]
    pub order_side: SideByRepr,
    #[serde(rename = "orderCancelReason")]
    pub order_cancel_reason: CancelReason,
    #[serde(rename = "orderId")]
    pub order_id: OrderId,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    #[serde(rename = "orderPrice")]
    pub order_price: Price,
    #[serde(rename = "initialSize")]
    pub initial_size: Size,
    #[serde(rename = "sizeOrVolumeLeft")]
    pub size_or_volume_left: Size,
    #[serde(rename = "createdAt")]
    pub created_at: Timestamp,
    #[serde(rename = "dealMoment")]
    pub deal_moment: Timestamp,
    #[serde(rename = "dealId")]
    pub deal_id: DealId,
    #[serde(rename = "dealAggressorSide")]
    pub deal_aggressor_side: SideByRepr,
    #[serde(rename = "dealPrice")]
    pub deal_price: Price,
    #[serde(rename = "dealSize")]
    pub deal_size: Size,
    #[serde(rename = "dealVolume")]
    pub deal_volume: Size,
    #[serde(rename = "dealDelta")]
    pub deal_delta: Size,
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    #[serde(rename = "isOrderByVolume")]
    pub is_order_by_volume: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsSettlementOrder {
    #[serde(rename = "settlementOrderId")]
    pub settlement_order_id: OrderId,
    pub currency1: String,
    pub currency2: String,
    pub size1: Size,
    pub size2: Size,
    #[serde(rename = "createdAt")]
    pub created_at: Timestamp,
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsSettlementDeal {
    #[serde(rename = "settlementOrderId")]
    pub settlement_order_id: OrderId,
    pub currency1: String,
    pub currency2: String,
    pub size1: Size,
    pub size2: Size,
    #[serde(rename = "createdAt")]
    pub created_at: Timestamp,
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    #[serde(rename = "settlementMoment")]
    pub settlement_moment: Timestamp,
    #[serde(rename = "settlementId")]
    pub settlement_id: DealId,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsGlobalLimit {
    #[serde(rename = "currencyName")]
    pub currency_name: String,
    #[serde(rename = "netLimit")]
    pub net_imit: Size,
    #[serde(rename = "grossLimit")]
    pub gross_imit: Size,
    #[serde(rename = "netLimitUtilization")]
    pub netlimit_utilization: Size,
    #[serde(rename = "grossLimitUtilization")]
    pub gross_limit_utilization: Size,
    pub flags: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsCounterpartyLimit {
    #[serde(rename = "currencyName")]
    pub currency_mame: String,
    #[serde(rename = "netLimit")]
    pub net_limit: Size,
    #[serde(rename = "grossLimit")]
    pub gross_limit: Size,
    #[serde(rename = "netLimitUtilization")]
    pub net_limit_utilization: Size,
    #[serde(rename = "grossLimitUtilization")]
    pub gross_limit_utilization: Size,
    pub flags: u16,
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    #[serde(rename = "takerMarkup")]
    pub taker_markup: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsCounterpartyMutualLimit {
    #[serde(rename = "currencyName")]
    pub currency_name: String,
    #[serde(rename = "netLimit")]
    pub net_limit: Size,
    #[serde(rename = "grossLimit")]
    pub gross_limit: Size,
    #[serde(rename = "netLimitUtilization")]
    pub net_imit_utilization: Size,
    #[serde(rename = "grossLimitUtilization")]
    pub gross_limit_utilization: Size,
    pub reserved1: u16,
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    pub reserved2: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsBookLevel {
    pub price: Price,
    pub size: Size,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum WsActionBookLevel {
    #[serde(rename = "+")]
    Added,
    #[serde(rename = "M")]
    Modified,
    #[serde(rename = "-")]
    Removed,
    #[serde(rename = "~")]
    RemovedSpecified,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsBookUpdate {
    pub action: WsActionBookLevel,
    pub price: Price,
    pub size: Size,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsSettlementRequest {
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    #[serde(rename = "currencyName")]
    pub currency_name: String,
    pub flags: u16,
    pub amount: Size,
    pub comment: String,
    #[serde(rename = "expirationTimestamp")]
    pub expiration_timestamp: Timestamp,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WsSettlementTransaction {
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    #[serde(rename = "currencyName")]
    pub currency_name: String,
    pub amount: Size,
    #[serde(rename = "settlementOrderId")]
    pub settlement_order_id: OrderId,
    pub comment: String,
    #[serde(rename = "createdAt")]
    pub created_at: Timestamp,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "sentAt")]
    pub sent_at: Timestamp,
    #[serde(rename = "transactionFlags")]
    pub transaction_flags: Flags,
    #[serde(rename = "transactionMoment")]
    pub transaction_moment: Timestamp,
    pub transaction: DealId,
    #[serde(rename = "networkFee")]
    pub network_fee: u16,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub fn test_serde_ws_request() {
        lunu_lib::logger::init_test();

        let json = r#"{"event": "bind", "feed": "I"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "I"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "P"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "P"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "G"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "G"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "L"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "L"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "M"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "M"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "B", "feedId": 123424235}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "bind", "feed": "B", "feedId": "BTC-EUR"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "B", "feedId": 123424235}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "F", "feedId": 123424235}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "bind", "feed": "F", "feedId": "BTC-EUR"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "F", "feedId": 123424235}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "R"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "R"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "N"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "N"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "K", "feedId": "EUR"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "K"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "O", "feedId": "BTC-EUR"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "O"}"#;
        test_serde_value_type::<WsRequest>(json);

        let json = r#"{"event": "bind", "feed": "S", "feedId": "EUR"}"#;
        test_serde_value_type::<WsRequest>(json);
        let json = r#"{"event": "unbind", "feed": "S"}"#;
        test_serde_value_type::<WsRequest>(json);
    }

    pub fn test_serde_ws_response_instrument() {
        // snapshot
        let json = r#"
        ["I", 0, "S", [
            "array of currency",
            "array of instrument"
        ]]
        "#;
        test_serde_value_type::<WsResponse>(json);

        // currency added
        let json = r#"["C", 0, "+", "currency"]"#;
        test_serde_value_type::<WsResponse>(json);

        // currency modified
        let json = r#"["C", 0, "M", "currency"]"#;
        test_serde_value_type::<WsResponse>(json);

        // currency removed
        let json = r#"["C", 0, "-", "currency"]"#;
        test_serde_value_type::<WsResponse>(json);

        // instrument added
        let json = r#"["I", 0, "+", "instrument"]"#;
        test_serde_value_type::<WsResponse>(json);

        // instrument modified
        let json = r#"["I", 0, "M", "instrument"]"#;
        test_serde_value_type::<WsResponse>(json);

        // instrument removed
        let json = r#"["I", 0, "-", "instrument"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["I", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["I", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_position_order() {
        // snapshot
        let json = r#"["P", 0, "S", [
            "nextDealOrSettlementOrTransactionId (Efx::DealId)",
            "array of position",
            "array of order",
            "array of settlementOrder"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // order added
        let json = r#"["O", 0, "+", "order"]"#;
        test_serde_value_type::<WsResponse>(json);

        // order executed
        let json = r#"["O", 0, "D", "deal"]"#;
        test_serde_value_type::<WsResponse>(json);

        // order removed
        let json = r#"["O", 0, "-", "order"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order added
        let json = r#"["S", 0, "+", "settlementOrder"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order modifed
        let json = r#"["S", 0, "M", "settlementOrder"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order executed
        let json = r#"["S", 0, "D", "settlementDeal"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order removed
        let json = r#"["S", 0, "-", "settlementOrder"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["P", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["P", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_global_limit() {
        // snapshot
        let json = r#"["G", 0, "S", "array of limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // limit modified
        let json = r#"["G", 0, "M", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["G", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["G", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_counterparty_limit() {
        // snapshot
        let json = r#"["L", 0, "S", "array of limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // limit added
        let json = r#"["L", 0, "+", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // limit modified
        let json = r#"["L", 0, "M", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // limit removed
        let json = r#"["L", 0, "-", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["L", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["L", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_counterparty_mutual_limit() {
        // snapshot
        let json = r#"["M", 0, "S", "array of limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // mutual limit added
        let json = r#"["M", 0, "+", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // mutual limit modified
        let json = r#"["M", 0, "M", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // mutual limit removed
        let json = r#"["M", 0, "-", "limit"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["M", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["M", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_global_order_book() {
        // snapshot
        let json = r#"["B", 123456789, "S", [
            "array of bid bookLevel",
            "array of ask boolLevel"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // book levels modified
        let json = r#"["B", 123456789, "M", [
            "array of bid bookUpdate",
            "array of ask bookUpdate"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // book levels added
        let json = r#"["B", 123456789, "M", [[["+", 300000000000, 10000000]], []]]"#;
        test_serde_value_type::<WsResponse>(json);

        // book level size modified
        let json = r#"["B", 123456789, "M", [[["M", 300000000000, 10000000]], []]]"#;
        test_serde_value_type::<WsResponse>(json);

        // book levels removed
        let json = r#"["B", 123456789, "M", [[["-", 300000000000, 10000000]], []]]"#;
        test_serde_value_type::<WsResponse>(json);

        // till specified price, price and size of a new top level are returned
        // (both zeros if book side became empty)
        let json = r#"["B", 123456789, "M", [[["~", 300000000000, 10000000]], []]]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["B", 123456789, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["B", 123456789, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_tradable_order_book() {
        let json = r#"["~", 300000000000, 10000000]"#;
        test_serde_value_type::<WsBookUpdate>(json);

        let json = r#"["~"]"#;
        test_serde_value_type::<WsBookUpdate>(json);
    }

    pub fn test_serde_ws_response_settlement_request() {
        // snapshot
        let json = r#"["R", 0, "S", [
            "array of incoming settlementRequest",
            "array of outgoing settlementRequest"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // add new settlement requests
        let json = r#"["R", 0, "+", [
            "array of new incoming settlementRequest",
            "array of new outgoing settlementRequest"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // remove settlement requests
        let json = r#"["R", 0, "-", [
            "array of removed incoming settlementRequest",
            "array of removed outgoing settlementRequest"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["R", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["R", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_settlement_transaction() {
        // snapshot
        let json = r#"["N", 0, "S", [
            "nextDealOrSettlementOrTransactionId (Efx::DealId)",
            "array of incoming settlementTransaction",
            "array of outgoing settlementTransaction"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // add settlement transactions
        let json = r#"["N", 0, "+", [
            "array of new incoming settlementTransaction",
            "array of new outgoing settlementTransaction"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // modify settlement transactions
        let json = r#"["N", 0, "M", [
            "array of received incoming settlementTransaction",
            "array of received outgoing settlementTransaction"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // commited settlement transactions
        let json = r#"["N", 0, "D", [
            "array of commited incoming settlementTransaction",
            "array of commited outgoing settlementTransaction"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // remove settlement transactions
        let json = r#"["N", 0, "-", [
            "array of removed incoming settlementTransaction",
            "array of removed outgoing settlementTransaction"
        ]]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["N", 0, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["N", 0, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_position_currency() {
        // snapshot
        let json = r#"["K", 123456789, "S", ["array of position"]]"#;
        test_serde_value_type::<WsResponse>(json);

        // new position
        let json = r#"["K", 123456789, "+", "position"]"#;
        test_serde_value_type::<WsResponse>(json);

        // update position
        let json = r#"["K", 123456789, "M", "position"]"#;
        test_serde_value_type::<WsResponse>(json);

        // del position
        let json = r#"["K", 123456789, "-", "position"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["K", 123456789, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["K", 123456789, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_order() {
        // snapshot
        let json = r#"["O", 123456789, "S", ["array of order"]]"#;
        test_serde_value_type::<WsResponse>(json);

        // new order
        let json = r#"["O", 123456789, "+", "order"]"#;
        test_serde_value_type::<WsResponse>(json);

        // del position
        let json = r#"["O", 123456789, "-", "order"]"#;
        test_serde_value_type::<WsResponse>(json);

        // new deal
        let json = r#"["O", 123456789, "D", "deal"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["O", 123456789, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["O", 123456789, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    pub fn test_serde_ws_response_settlement_order() {
        // snapshot
        let json = r#"["S", 123456789, "S", ["array of settlementOrder"]]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order added
        let json = r#"["S", 123456789, "+", "settlementOrder"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order modifed
        let json = r#"["S", 123456789, "M", "settlementOrder"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order executed
        let json = r#"["S", 123456789, "D", "settlementDeal"]"#;
        test_serde_value_type::<WsResponse>(json);

        // settlement order removed
        let json = r#"["S", 123456789, "-", "settlementOrder"]"#;
        test_serde_value_type::<WsResponse>(json);

        // failed to subscribe
        let json = r#"["S", 123456789, "Z", 2]"#;
        test_serde_value_type::<WsResponse>(json);

        // unsubscribed
        let json = r#"["S", 123456789, "U", 0]"#;
        test_serde_value_type::<WsResponse>(json);
    }

    #[test]
    pub fn test_serde_ws_response() {
        test_serde_ws_response_instrument();
        test_serde_ws_response_position_order();
        test_serde_ws_response_global_limit();
        test_serde_ws_response_counterparty_limit();
        test_serde_ws_response_counterparty_mutual_limit();
        test_serde_ws_response_global_order_book();
        test_serde_ws_response_tradable_order_book();
        test_serde_ws_response_settlement_request();
        test_serde_ws_response_settlement_transaction();
        test_serde_ws_response_position_currency();
        test_serde_ws_response_order();
        test_serde_ws_response_settlement_order();
    }
}
