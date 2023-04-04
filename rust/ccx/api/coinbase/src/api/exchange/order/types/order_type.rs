use crate::api::exchange::prelude::*;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    /// A [limit order](https://en.wikipedia.org/wiki/Order_(exchange)#Limit_order).
    Limit,
    /// A [market order](https://en.wikipedia.org/wiki/Order_(exchange)#Market_order).
    Market,
    /// A [stop order](https://en.wikipedia.org/wiki/Order_(exchange)#Stop_orders).
    Stop,
}
#[cfg(feature = "db")]
forward_display_to_serde!(OrderType);
#[cfg(feature = "db")]
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
