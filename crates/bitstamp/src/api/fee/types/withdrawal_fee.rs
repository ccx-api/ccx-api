use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Clone, Debug, Deserialize)]
pub struct WithdrawalFee {
    pub currency: Atom,
    pub fee: Decimal,
}
