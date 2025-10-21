#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum PortfolioWalletType {
    /// A trading wallet.
    #[serde(rename = "TRADING")]
    Trading,
    /// A crypto vault.
    #[serde(rename = "VAULT")]
    Vault,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(AccountPortfolioWalletType);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(AccountPortfolioWalletType);

// impl PortfolioWalletType {
//     pub fn from_name(name: &str) -> Option<Self> {
//         Self::from_str(name).ok()
//     }
//
//     pub fn name(&self) -> String {
//         self.to_string()
//     }
// }
