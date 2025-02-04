use serde::Deserialize;
use serde::Serialize;

use crate::env_var_with_prefix;

#[derive(Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub host: String,
    pub port: u16,
}

impl Proxy {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn from_env_with_prefix(prefix: &str) -> Option<Self> {
        let host = env_var_with_prefix(prefix, "PROXY_HOST")?;
        let port = env_var_with_prefix(prefix, "PROXY_PORT")?;
        let port = port.parse::<u16>().ok()?;
        Some(Proxy { host, port })
    }
}
