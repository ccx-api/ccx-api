use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileInfo {
    id: String,
    user_id: String,
    name: String,
    active: bool,
    is_default: bool,
    created_at: String,
}
