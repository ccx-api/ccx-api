use ccx_api_lib::env_var_with_prefix;
use serde::Deserialize;
use serde::Serialize;

pub mod dt_gatepay;
pub mod maybe_str;

/// Gatepay API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GatepayApiCred {
    pub api_key: String,
    pub client_id: String,
    pub auth_key: String,
}

impl GatepayApiCred {
    pub fn new(
        api_key: Option<String>,
        client_id: Option<String>,
        auth_key: Option<String>,
    ) -> Self {
        GatepayApiCred {
            api_key: api_key.unwrap_or_default(),
            client_id: client_id.unwrap_or_default(),
            auth_key: auth_key.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_API_KEY", "${prefix}_CLIENT_ID", and "${prefix}_AUTH_KEY"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        GatepayApiCred::new(
            env_var_with_prefix(prefix, "API_KEY"),
            env_var_with_prefix(prefix, "CLIENT_ID"),
            env_var_with_prefix(prefix, "AUTH_KEY"),
        )
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GatepayNotificationCred {
    pub client_id: String,
    pub payment_key: String,
}

impl GatepayNotificationCred {
    pub fn new(client_id: Option<String>, payment_key: Option<String>) -> Self {
        GatepayNotificationCred {
            client_id: client_id.unwrap_or_default(),
            payment_key: payment_key.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_CLIENT_ID" and "${prefix}_PAYMENT_KEY"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        GatepayNotificationCred::new(
            env_var_with_prefix(prefix, "CLIENT_ID"),
            env_var_with_prefix(prefix, "PAYMENT_KEY"),
        )
    }
}
