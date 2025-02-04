use serde::Deserialize;

use crate::Atom;

#[derive(Clone, Debug, Deserialize)]
pub struct MyTradingPair {
    pub name: Atom,
    pub url_symbol: Atom,
}
