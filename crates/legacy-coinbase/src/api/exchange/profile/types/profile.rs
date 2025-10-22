use crate::api::exchange::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Atom,
    pub name: Atom,
    pub active: bool,
    pub is_default: bool,
    pub created_at: DtCoinbaseEx,
}
