#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;

use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum PortfolioOrderSide {
    /// Buy order.
    #[serde(rename = "BUY")]
    Buy,
    /// Sell order.
    #[serde(rename = "SELL")]
    Sell,
}
#[cfg(feature = "db")]
forward_display_to_serde!(PortfolioOrderSide);
#[cfg(feature = "db")]
forward_from_str_to_serde!(PortfolioOrderSide);

// impl PortfolioOrderSide {
//     pub fn from_name(name: &str) -> Option<Self> {
//         Self::from_str(name).ok()
//     }
//
//     pub fn name(&self) -> String {
//         self.to_string()
//     }
// }
