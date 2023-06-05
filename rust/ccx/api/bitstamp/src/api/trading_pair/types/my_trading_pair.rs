use serde::Deserialize;

use crate::Atom;

#[derive(Debug, Deserialize)]
pub struct MyTradingPair {
    pub name: Atom,
    pub url_symbol: Atom,
}
