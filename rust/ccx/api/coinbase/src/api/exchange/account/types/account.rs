use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub currency: Atom,
    pub balance: Decimal,
    pub hold: Decimal,
    pub available: Decimal,
    pub profile_id: Uuid,
    pub trading_enabled: bool,
}
