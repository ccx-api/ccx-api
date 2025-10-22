use crate::DtCoinbasePrime;
use crate::api::prime::PortfolioWalletType;
use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioWallet {
    pub id: Uuid,
    pub name: String,
    pub symbol: Atom,
    pub r#type: PortfolioWalletType,
    pub created_at: DtCoinbasePrime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
