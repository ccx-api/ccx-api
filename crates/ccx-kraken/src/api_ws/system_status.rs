use serde::Deserialize;

use crate::api::spot::market::Status;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct StatusUpdate {
    pub version: String,
    pub system: Status,
    pub api_version: String,
}
