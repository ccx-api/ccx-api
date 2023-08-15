#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderTimeInForce {
    /// Good till cancelled.
    Gtc,
    /// Good till time.
    Gtt,
    /// Immediate or cancel.
    Ioc,
    /// Fill or kill.
    Fok,
}
#[cfg(feature = "db")]
forward_display_to_serde!(OrderTimeInForce);
#[cfg(feature = "db")]
forward_from_str_to_serde!(OrderTimeInForce);

impl OrderTimeInForce {
    /// Returns the string representation of the `OrderTimeInForce` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderTimeInForce::Gtc => "GTC",
            OrderTimeInForce::Gtt => "GTT",
            OrderTimeInForce::Ioc => "IOC",
            OrderTimeInForce::Fok => "FOK",
        }
    }
}

impl AsRef<str> for OrderTimeInForce {
    /// Returns a reference to the string representation of the `OrderTimeInForce` enum value.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
