use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, derive_more::Error, derive_more::Display,
)]
#[display("{error_name}: {error}")]
#[serde(rename_all = "camelCase")]
pub struct BitGoApiError {
    /// Human-readable error message
    error: String,
    /// Contains error code
    #[serde(default = "default_error_name", alias = "name")]
    error_name: String,
    #[serde(alias = "reqId")]
    request_id: Option<String>,
}

fn default_error_name() -> String {
    "UnknownError".to_string()
}
