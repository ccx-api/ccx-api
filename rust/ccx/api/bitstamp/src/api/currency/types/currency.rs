use serde::Deserialize;

use crate::maybe_str;
use crate::Atom;
use crate::Decimal;

#[derive(Debug, Deserialize)]
pub struct Currency {
    #[serde(with = "maybe_str")]
    pub available_supply: Option<Decimal>,
    pub currency: Atom,
    pub decimals: u8,
    pub logo: Atom,
    pub name: Atom,
    pub symbol: Option<Atom>,
    pub r#type: Type,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Crypto,
    Fiat,
}
