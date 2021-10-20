use serde::{Deserialize, Serialize};

use crate::env_var_with_prefix;

/// API credentials.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiCred {
    pub key: String,
    pub secret: String,
}

impl ApiCred {
    pub fn new(key: Option<String>, secret: Option<String>) -> Self {
        ApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        ApiCred::new(
            env_var_with_prefix(prefix, "KEY"),
            env_var_with_prefix(prefix, "SECRET"),
        )
    }
}
