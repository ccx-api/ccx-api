use super::filter::*;
use crate::api::um::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: Atom,
    pub pair: Atom,
    pub contract_type: ContractType,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub status: ContractStatus,
    /// Ignore
    pub maint_margin_percent: Decimal,
    /// Ignore
    pub required_margin_percent: Decimal,
    pub base_asset: Atom,
    pub quote_asset: Atom,
    pub margin_asset: Atom,
    /// Please do not use it as tickSize.
    pub price_precision: u16,
    /// Please do not use it as tickSize.
    pub quantity_precision: u16,
    pub base_asset_precision: u16,
    pub underlying_type: UnderlyingType,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: u64,
    /// Threshold for algo order with "priceProtect".
    pub trigger_protect: Decimal,
    pub filters: Vec<Filter>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SumbolType {
    #[serde(rename = "FUTURE")]
    Future,
}

// TODO check variants
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ContractType {
    #[serde(rename = "")]
    Unknown,
    #[serde(rename = "PERPETUAL")]
    Perpetual,
    #[serde(rename = "CURRENT_MONTH")]
    CurrentMonth,
    #[serde(rename = "NEXT_MONTH")]
    NextMonth,
    /// 当季交割合约
    #[serde(rename = "CURRENT_QUARTER")]
    CurrentQuarter,
    /// 次季交割合约
    #[serde(rename = "NEXT_QUARTER")]
    NextQarter,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ContractStatus {
    #[serde(rename = "PENDING_TRADING")]
    PendingTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "PRE_DELIVERING")]
    PreDelivering,
    #[serde(rename = "DELIVERING")]
    Delivering,
    #[serde(rename = "DELIVERED")]
    Delivered,
    #[serde(rename = "PRE_SETTLE")]
    PreSettle,
    #[serde(rename = "SETTLING")]
    Settling,
    #[serde(rename = "CLOSE")]
    Close,
}

// TODO check variants
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UnderlyingType {
    #[serde(rename = "COIN")]
    Coin,
    #[serde(rename = "INDEX")]
    Index,
}

// TODO check variants
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "STOP_MARKET")]
    StopMarket,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[serde(rename = "TRAILING_STOP_MARKET")]
    TrailingStopMarket,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TimeInForce {
    /// Good Til Canceled
    /// An order will be on the book unless the order is canceled.
    #[serde(rename = "GTC")]
    GoodTilCanceled,
    /// Immediate Or Cancel
    /// An order will try to fill the order as much as it can before the order expires.
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    /// Fill or Kill
    /// An order will expire if the full order cannot be filled upon execution.
    #[serde(rename = "FOK")]
    FillOrKill,
    /// Good Till Crossing (Post Only)
    #[serde(rename = "GTX")]
    GoodTilCrossing,
}
