use std::fmt;
use std::io;
use std::time;

use thiserror::Error;

#[cfg(any(feature = "with_awc", feature = "with_reqwest"))]
use self::with_network::*;

#[cfg(any(feature = "with_awc", feature = "with_reqwest"))]
mod with_network {
    #[cfg(feature = "with_awc")]
    pub use actix_http::ws::ProtocolError;
    #[cfg(feature = "with_awc")]
    pub use awc::error::JsonPayloadError;
    #[cfg(feature = "with_awc")]
    pub use awc::error::PayloadError;
    #[cfg(feature = "with_awc")]
    pub use awc::error::SendRequestError as AwcSendRequestError;
    #[cfg(feature = "with_awc")]
    pub use awc::error::WsClientError;
    #[cfg(feature = "with_awc")]
    pub use awc::http::header::InvalidHeaderValue as AwcInvalidHeaderValue;
    #[cfg(feature = "with_reqwest")]
    pub use reqwest::Error as ReqwestError;
    #[cfg(feature = "with_reqwest")]
    pub use reqwest::header::InvalidHeaderValue as ReqwestInvalidHeaderValue;
}

#[derive(Clone, Debug, Error)]
pub enum ApiServiceError {
    #[error("Server Error")]
    ServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Rate Limit Exceeded")]
    RateLimitExceeded,
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
    ServiceError(#[from] ApiServiceError),
    #[cfg(feature = "with_awc")]
    #[error("Unknown Status: {0}")]
    UnknownStatusAwc(awc::http::StatusCode),
    #[cfg(feature = "with_reqwest")]
    #[error("Unknown Status: {0}")]
    UnknownStatusReqwest(reqwest::StatusCode),
    // Backward compatibility alias
    #[cfg(feature = "with_awc")]
    #[error("Unknown Status: {0}")]
    UnknownStatus(awc::http::StatusCode),
    #[cfg(any(feature = "with_awc", feature = "with_reqwest"))]
    #[error("Request Error: {0}")]
    RequestError(String),
    #[cfg(feature = "with_awc")]
    #[error("Invalid Header: {0}")]
    InvalidHeaderErrorAwc(#[from] AwcInvalidHeaderValue),
    #[cfg(feature = "with_reqwest")]
    #[error("Invalid Header: {0}")]
    InvalidHeaderErrorReqwest(#[from] ReqwestInvalidHeaderValue),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Url Parse Error: {0}")]
    UrlParserError(#[from] url::ParseError),
    #[error("Url Encoded Error: {0}")]
    UrlEncodedError(#[from] serde_urlencoded::ser::Error),
    #[error("Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "with_awc")]
    #[error("Payload Error: {0}")]
    Payload(#[from] PayloadError),
    #[cfg(feature = "with_awc")]
    #[error("Json Payload Error: {0}")]
    JsonPayload(#[from] JsonPayloadError),
    #[error("Time Error: {0}")]
    TimestampError(#[from] time::SystemTimeError),
    #[cfg(feature = "with_awc")]
    #[error("Websocket Client Error: {0}")]
    WsClientError(String),
    #[cfg(feature = "with_awc")]
    #[error("Websocket Protocol Error: {0}")]
    WsProtocolError(#[from] ProtocolError),
    #[error("Sign Error: {0}")]
    SignError(#[from] SignError),
    #[error("Other Error: {0}")]
    Other(String),
}

#[cfg(feature = "with_awc")]
impl<AE> From<AwcSendRequestError> for LibError<AE>
where
    AE: CcxApiError + 'static,
{
    fn from(e: AwcSendRequestError) -> Self {
        Self::RequestError(e.to_string())
    }
}

#[cfg(feature = "with_reqwest")]
impl<AE> From<ReqwestError> for LibError<AE>
where
    AE: CcxApiError + 'static,
{
    fn from(e: ReqwestError) -> Self {
        Self::RequestError(e.to_string())
    }
}

#[cfg(feature = "with_awc")]
impl<AE> From<WsClientError> for LibError<AE>
where
    AE: CcxApiError + 'static,
{
    fn from(e: WsClientError) -> Self {
        Self::WsClientError(e.to_string())
    }
}

impl<AE> LibError<AE>
where
    AE: CcxApiError + 'static,
{
    pub fn other(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}

pub trait CcxApiError: std::error::Error + Send + Sync {}
