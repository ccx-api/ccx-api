#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::exchange::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    /// The product is online and available for trading. (Default).
    Online,
    /// The product is offline and not available for trading.
    Offline,
    /// The product is currently undergoing an internal review.
    Internal,
    /// The product has been delisted and is no longer available for trading.
    Delisted,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(ProductStatus);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(ProductStatus);

impl ProductStatus {
    /// Returns the string representation of the `ProductStatus` enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            ProductStatus::Online => "online",
            ProductStatus::Offline => "offline",
            ProductStatus::Internal => "internal",
            ProductStatus::Delisted => "delisted",
        }
    }
}

impl AsRef<str> for ProductStatus {
    /// Returns a reference to the string representation of the `ProductStatus` enum value.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
