use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::Atom;
use crate::DtCoinbase;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Atom,
    pub active: bool,
    pub is_default: bool,
    pub created_at: DtCoinbase,
}
