use crate::api::exchange::prelude::*;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    /// Buy order.
    Buy,
    /// Sell order.
    Sell,
}
#[cfg(feature = "db")]
forward_display_to_serde!(OrderSide);
#[cfg(feature = "db")]
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
