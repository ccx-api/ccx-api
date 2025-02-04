use super::prelude::*;

mod oflags;
mod txids;

pub use self::oflags::*;
pub use self::txids::*;

/// Which time to use to search.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum CloseTime {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "close")]
    Close,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum OrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum OrderStatus {
    /// Order pending book entry.
    #[serde(rename = "pending")]
    Pending,
    /// Open order.
    #[serde(rename = "open")]
    Open,
    /// Closed order.
    #[serde(rename = "closed")]
    Closed,
    /// Order canceled.
    #[serde(rename = "canceled")]
    Canceled,
    /// Order expired.
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum OrderType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop-loss")]
    StopLoss,
    #[serde(rename = "take-profit")]
    TakeProfit,
    #[serde(rename = "stop-loss-limit")]
    StopLossLimit,
    #[serde(rename = "take-profit-limit")]
    TakeProfitLimit,
    #[serde(rename = "settle-position")]
    SettlePosition,
}

/// Time-in-force of the order to specify how long it should remain in the order book before being
/// cancelled.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum TimeInForce {
    /// GTC (Good-'til-cancelled) is default if the parameter is omitted.
    #[serde(rename = "GTC")]
    Gtc,
    /// GTD (good-'til-date), if specified, must coincide with a desired expiretm.
    #[serde(rename = "GTD")]
    Gtd,
    /// IOC (immediate-or-cancel) will immediately execute the amount possible and cancel
    /// any remaining balance rather than resting in the book.
    #[serde(rename = "IOC")]
    Ioc,
}
