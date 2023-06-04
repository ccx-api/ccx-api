use serde::Deserialize;

use crate::Decimal;

#[derive(Debug, Deserialize)]
pub struct AccountBalance {
    pub currency: String,
    pub total: Decimal,
    pub available: Decimal,
    pub reserved: Decimal,
}
