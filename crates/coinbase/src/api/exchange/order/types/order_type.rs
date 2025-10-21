#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    /// A [limit order](https://en.wikipedia.org/wiki/Order_(exchange)#Limit_order).
    Limit,
    /// A [market order](https://en.wikipedia.org/wiki/Order_(exchange)#Market_order).
    Market,
    /// A [stop order](https://en.wikipedia.org/wiki/Order_(exchange)#Stop_orders).
    Stop,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(OrderType);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(OrderType);

impl OrderType {
    /// Returns the string representation of the `OrderType` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::Stop => "stop",
        }
    }
}

impl AsRef<str> for OrderType {
    /// Returns a reference to the string representation of the `OrderType` enum value.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
