#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

/// Possible values for cancel after in Coinbase Exchange/Pro API.
///
/// Requires time_in_force to be GTT.
///
/// GTT "Good till time" orders remain open on the book until canceled or the allotted cancel_after
/// is depleted on the matching engine. GTT orders are guaranteed to cancel before any other order
/// is processed after the cancel_after timestamp which is returned by the API.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum CancelAfter {
    /// Cancel after a minute.
    Min,
    /// Cancel after a hour.
    Hour,
    /// Cancel after a day. A day is considered 24 hours.
    Day,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(CancelAfter);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(CancelAfter);

impl CancelAfter {
    /// Returns the string representation of the `CancelAfter` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            CancelAfter::Min => "min",
            CancelAfter::Hour => "hour",
            CancelAfter::Day => "day",
        }
    }
}

impl AsRef<str> for CancelAfter {
    /// Returns a reference to the string representation of the `CancelAfter` enum value.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
