#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

/// Self-Trade Prevention
///
/// Self-trading is not allowed on the exchange. Two orders from the same user are not allowed
/// to match with one another. To change the self-trade behavior, specify the stp flag.
///
/// See the self-trade prevention documentation for details about these fields.
/// [https://docs.cloud.coinbase.com/exchange/docs/matching-engine#self-trade-prevention]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum OrderStp {
    /// Cancel smaller order and decrement larger order by the smaller size.
    /// If the same size, cancel both.
    /// (Default).
    #[serde(rename = "dc")]
    DecreaseAndCancel,
    /// Cancel older (resting) order in full. Continue to execute the newer taking order.
    #[serde(rename = "co")]
    CancelOldest,
    /// Cancel newer (taking) order in full. Let the old resting order remain on the order book.
    #[serde(rename = "cn")]
    CancelNewest,
    /// Cancel both orders immediately.
    #[serde(rename = "cb")]
    CancelBoth,
}
#[cfg(feature = "db")]
forward_display_to_serde!(OrderStp);
#[cfg(feature = "db")]
forward_from_str_to_serde!(OrderStp);

impl OrderStp {
    /// Returns the string representation of the `OrderStp` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStp::DecreaseAndCancel => "dc",
            OrderStp::CancelOldest => "co",
            OrderStp::CancelNewest => "cn",
            OrderStp::CancelBoth => "cb",
        }
    }
}

impl AsRef<str> for OrderStp {
    /// Returns a reference to the string representation of the `OrderStp` enum value.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
