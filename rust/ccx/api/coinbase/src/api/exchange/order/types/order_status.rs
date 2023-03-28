#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// The order is open but unfilled.
    Open,
    /// The order has been sent but is not yet confirmed.
    Pending,
    /// The order has been rejected.
    Rejected,
    Done,
    Active,
    Received,
    All,
}
#[cfg(feature = "db")]
forward_display_to_serde!(OrderStatus);
#[cfg(feature = "db")]
forward_from_str_to_serde!(OrderStatus);

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Open => "open",
            OrderStatus::Pending => "pending",
            OrderStatus::Rejected => "rejected",
            OrderStatus::Done => "done",
            OrderStatus::Active => "active",
            OrderStatus::Received => "received",
            OrderStatus::All => "all",
        }
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
