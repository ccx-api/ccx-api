use crate::api::prime::prelude::*;
use crate::api::prime::PortfolioWalletType;
use crate::dt_coinbase::DtCoinbase;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWallet {
    pub id: Uuid,
    pub name: String,
    pub symbol: Atom,
    pub r#type: PortfolioWalletType,
    pub created_at: DtCoinbase,
}
