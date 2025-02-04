use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Clone, Debug, Deserialize)]
pub struct TradingFee {
    pub currency_pair: Atom,
    pub fees: Fees,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Fees {
    pub maker: Decimal,
    pub taker: Decimal,
}
