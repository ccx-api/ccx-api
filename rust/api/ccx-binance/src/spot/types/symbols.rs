use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use smart_string::SmartString;

use crate::spot::types::filters::Filter;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: SymbolName,
    pub status: SymbolStatus,
    pub base_asset: AssetName,
    pub base_asset_precision: u16,
    pub quote_asset: AssetName,
    #[deprecated(note = "will be removed in future api versions (v4+)")]
    pub quote_precision: Option<u16>,
    pub quote_asset_precision: u16,
    pub base_commission_precision: u16,
    pub quote_commission_precision: u16,
    // FIXME pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filter>,
    pub permissions: Vec<SymbolPermission>,
}

pub type SymbolName = SmartString;
pub type AssetName = SmartString;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SymbolStatus {
    #[serde(rename = "PRE_TRADING")]
    PreTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "POST_TRADING")]
    PostTrading,
    #[serde(rename = "END_OF_DAY")]
    EndOfDay,
    #[serde(rename = "HALT")]
    Halt,
    #[serde(rename = "AUCTION_MATCH")]
    AuctionMatch,
    #[serde(rename = "BREAK")]
    Break,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SymbolPermission {
    Spot,
    Margin,
    Leveraged,
    TradeGroup(u16),
}

impl Serialize for SymbolPermission {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SymbolPermission::Spot => s.serialize_str("SPOT"),
            SymbolPermission::Margin => s.serialize_str("MARGIN"),
            SymbolPermission::Leveraged => s.serialize_str("LEVERAGED"),
            SymbolPermission::TradeGroup(group_num) => {
                let group_num = format!("TRD_GRP_{:0>4}", group_num);
                s.serialize_str(&group_num)
            }
        }
    }
}

impl<'de> Deserialize<'de> for SymbolPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match &*s {
            "SPOT" => Ok(Self::Spot),
            "MARGIN" => Ok(Self::Margin),
            "LEVERAGED" => Ok(Self::Leveraged),
            trade_group if trade_group.contains("TRD_GRP") => {
                // Format: TRD_GRP_0001
                let group_num = trade_group.trim_start_matches("TRD_GRP_");
                let group_num = group_num.parse::<u16>().map_err(de::Error::custom)?;
                Ok(Self::TradeGroup(group_num))
            }
            _ => Err(de::Error::custom(format!("invalid type: {}", s))),
        }
    }
}
