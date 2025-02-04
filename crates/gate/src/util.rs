use ccx_api_lib::env_var_with_prefix;
use serde::Deserialize;
use serde::Serialize;

/// Gate.io API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GateApiCred {
    pub key: String,
    pub secret: String,
}

impl GateApiCred {
    pub fn new(key: Option<String>, secret: Option<String>) -> Self {
        GateApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY", and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        GateApiCred::new(
            env_var_with_prefix(prefix, "KEY"),
            env_var_with_prefix(prefix, "SECRET"),
        )
    }
}
