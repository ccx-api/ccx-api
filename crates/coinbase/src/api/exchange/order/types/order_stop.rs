#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum OrderStop {
    Loss,
    Entry,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(OrderStop);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(OrderStop);

impl OrderStop {
    /// Returns the string representation of the `OrderStop` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStop::Loss => "loss",
            OrderStop::Entry => "entry",
        }
    }
}

impl AsRef<str> for OrderStop {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
