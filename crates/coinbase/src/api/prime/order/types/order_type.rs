#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum PortfolioOrderType {
    /// A [market order](https://en.wikipedia.org/wiki/Order_(exchange)#Market_order).
    #[serde(rename = "MARKET")]
    Market,
    /// A [limit order](https://en.wikipedia.org/wiki/Order_(exchange)#Limit_order).
    #[serde(rename = "LIMIT")]
    Limit,
    /// A [time-weighted average price order](https://en.wikipedia.org/wiki/Time-weighted_average_price).
    #[serde(rename = "TWAP")]
    Twap,
    /// A [block trade](https://en.wikipedia.org/wiki/Block_trade).
    #[serde(rename = "BLOCK")]
    Block,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(PortfolioOrderType);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(PortfolioOrderType);

// impl PortfolioOrderType {
//     pub fn from_name(name: &str) -> Option<Self> {
//         Self::from_str(name).ok()
//     }
//
//     pub fn name(&self) -> String {
//         self.to_string()
//     }
// }
