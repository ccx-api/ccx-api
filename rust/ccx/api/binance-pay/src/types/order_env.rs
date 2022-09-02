use serde::Deserialize;
use serde::Serialize;

use crate::types::enums::OsType;
use crate::types::enums::TerminalType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderEnv {
    #[serde(rename = "terminalType")]
    pub terminal_type: TerminalType,
    #[serde(rename = "osType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[serde(rename = "orderClientIp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_client_ip: Option<String>,
    #[serde(rename = "cookieId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookie_id: Option<String>,
}

impl From<TerminalType> for OrderEnv {
    fn from(terminal_type: TerminalType) -> Self {
        Self {
            terminal_type,
            os_type: None,
            order_client_ip: None,
            cookie_id: None,
        }
    }
}
