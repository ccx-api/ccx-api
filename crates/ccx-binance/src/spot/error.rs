use std::borrow::Cow;
use serde::Deserialize;
use serde::Serialize;

use crate::spot::meta::BinanceSpotMeta;

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
    #[error("Failed to sign: {0}")]
    Sign(Cow<'static, str>),
    #[error("Decoding error: {0}")]
    Decoding(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(#[from] BinanceSpotErrorResponse),
}

impl BinanceSpotError {
    pub fn sign(msg: impl Into<Cow<'static, str>>) -> Self {
        BinanceSpotError::Sign(msg.into())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Binance Spot Send Request Error: {error}")]
pub struct BinanceSpotSendError {
    pub error: BinanceSpotError,
    pub meta: Option<BinanceSpotMeta>,
}

impl BinanceSpotSendError {
    pub fn new(error: BinanceSpotError, meta: Option<BinanceSpotMeta>) -> Self {
        BinanceSpotSendError { error, meta }
    }
}

impl<T: Into<BinanceSpotError>> From<T> for BinanceSpotSendError {
    fn from(error: T) -> Self {
        BinanceSpotSendError {
            error: error.into(),
            meta: None,
        }
    }
}
