#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    /// Buy order.
    Buy,
    /// Sell order.
    Sell,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(OrderSide);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(OrderSide);

impl OrderSide {
    /// Returns the string representation of the `OrderSide` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
    }
}

impl AsRef<str> for OrderSide {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
