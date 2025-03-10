use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_string::SmartString;

use crate::types::filters::Filter;

use super::order::OrderType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: SmartString,
    pub status: SymbolStatus,
    pub base_asset: SmartString,
    pub base_asset_precision: u16,
    pub quote_asset: SmartString,
    pub quote_precision: u16,
    pub quote_asset_precision: u16,
    pub base_commission_precision: u16,
    pub quote_commission_precision: u16,
    pub order_types: Vec<OrderType>,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub quote_amount_precision: SmartString,
    pub base_size_precision: SmartString,
    pub permissions: Vec<SymbolPermission>,
    pub filters: Vec<Filter>,
    pub max_quote_amount: String,
    pub maker_commission: SmartString,
    pub taker_commission: SmartString,
    pub quote_amount_precision_market: SmartString,
    pub max_quote_amount_market: String,
    pub full_name: SmartString,
    pub trade_side_type: TradeSideType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
pub enum SymbolStatus {
    #[serde(rename = "1")]
    Online,
    #[serde(rename = "2")]
    Pause,
    #[serde(rename = "3")]
    Offline,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum TradeSideType {
    All = 1,
    BuyOrderOnly = 2,
    SellOrderOnly = 3,
    Close = 4,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolPermission {
    Spot,
}
