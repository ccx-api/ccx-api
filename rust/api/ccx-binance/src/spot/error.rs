use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
#[error("Binance Spot Error: {code} - {msg}")]
pub struct BinanceSpotErrorResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BinanceSpotError {
    #[error("Failed to format argument: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("Failed to serialize query: {0}")]
    Query(#[from] serde_urlencoded::ser::Error),
    #[error("Connection error: {0}")]
    Connection(#[from] reqwest::Error),
    #[error("Authentication error")]
    Authentication,
    #[error("Other error: {0}")]
    Other(#[from] BinanceSpotErrorResponse),
}
