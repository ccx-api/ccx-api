use crate::api::prime::prelude::*;
use crate::api::prime::PortfolioWalletType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWallet {
    pub id: Uuid,
    pub name: String,
    pub symbol: Atom,
    pub r#type: PortfolioWalletType,
    // TODO time type
    pub created_at: String,
}
