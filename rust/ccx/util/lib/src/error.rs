use std::{fmt, io, time};

use serde_json;
use thiserror::Error;
use url;

#[cfg(feature = "with_network")]
use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    pub use awc::error::JsonPayloadError;
    pub use awc::error::PayloadError;
    pub use awc::error::SendRequestError;
    pub use awc::error::WsClientError;
    pub use awc::error::WsProtocolError;
    pub use awc::http::header::InvalidHeaderValue;
}

#[derive(Clone, Debug, Error)]
pub enum ServiceError {
    #[error("Server Error")]
    ServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
}

#[derive(Debug, Error)]
pub struct SignError {
    pub reason: String,
}

impl SignError {
    pub fn new(s: impl Into<String>) -> Self {
        Self { reason: s.into() }
    }
}

impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SignError {{ reason: {} }}", self.reason)
    }
}

pub type LibResult<T, AE> = std::result::Result<T, LibError<AE>>;

#[derive(Debug, Error)]
pub enum LibError<AE>
where
    AE: CcxApiError + 'static,
{
    #[error("Client Error: {0}")]
    ApiError(#[from] AE),
    #[error("Service Error: {0}")]
    ServiceError(#[from] ServiceError),
    #[cfg(feature = "with_network")]
    #[error("Unknown Status: {0}")]
    UnknownStatus(awc::http::StatusCode),
    #[cfg(feature = "with_network")]
    #[error("Request Error: {0}")]
    RequestError(#[from] SendRequestError),
    #[cfg(feature = "with_network")]
    #[error("Invalid Header: {0}")]
    InvalidHeaderError(#[from] InvalidHeaderValue),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Url Parse Error: {0}")]
    UrlParserError(#[from] url::ParseError),
    #[error("Url Encoded Error: {0}")]
    UrlEncodedError(#[from] serde_urlencoded::ser::Error),
    #[error("Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "with_network")]
    #[error("Payload Error: {0}")]
    Payload(#[from] PayloadError),
    #[cfg(feature = "with_network")]
    #[error("Json Payload Error: {0}")]
    JsonPayload(#[from] JsonPayloadError),
    #[error("Time Error: {0}")]
    TimestampError(#[from] time::SystemTimeError),
    #[cfg(feature = "with_network")]
    #[error("Websocket Client Error: {0}")]
    WsClientError(#[from] WsClientError),
    #[cfg(feature = "with_network")]
    #[error("Websocket Protocol Error: {0}")]
    WsProtocolError(#[from] WsProtocolError),
    #[error("Sign Error: {0}")]
    SignError(#[from] SignError),
    #[error("Other Error: {0}")]
    Other(String),
}

impl<AE> LibError<AE>
where
    AE: CcxApiError + 'static,
{
    pub fn other(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}

pub trait CcxApiError: std::error::Error {}
