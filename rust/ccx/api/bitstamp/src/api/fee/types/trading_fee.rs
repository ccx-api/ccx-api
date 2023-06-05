use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Deserialize)]
pub struct TradingFee {
    pub currency_pair: Atom,
    pub fees: Fees,
}

#[derive(Debug, Deserialize)]
pub struct Fees {
    pub maker: Decimal,
    pub taker: Decimal,
}
