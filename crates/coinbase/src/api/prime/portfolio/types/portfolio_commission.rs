use derive_more::Deref;
use serde::Deserialize;
use serde::Serialize;

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Deref)]
pub struct PortfolioCommission {
    pub commission: PortfolioCommissionDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioCommissionDetails {
    pub r#type: Atom,
    pub rate: Decimal,
    pub trading_volume: Decimal,
}
