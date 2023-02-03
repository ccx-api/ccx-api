use actix::MailboxError;
use awc::error::PayloadError;
use awc::error::SendRequestError;
use awc::http::header::InvalidHeaderValue;
use thiserror::Error;

use crate::error::ApiFineryError;

pub type LibResult<T> = std::result::Result<T, LibError>;

#[derive(Clone, Debug, Error)]
pub enum ServiceError {
    #[error("Server Error")]
    ServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
}

#[derive(Debug, Error)]
pub enum LibError {
    #[error("Api Finery Error: {0}")]
    ApiError(#[from] ApiFineryError),
    #[error("Url Parse Error: {0}")]
    UrlParserError(#[from] url::ParseError),
    #[error("Invalid Header: {0}")]
    InvalidHeaderError(#[from] InvalidHeaderValue),
    #[error("Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Request Error: {0}")]
    RequestError(#[from] SendRequestError),
    #[error("Payload Error: {0}")]
    Payload(#[from] PayloadError),
    #[error("Unknown Status: {0}")]
    UnknownStatus(awc::http::StatusCode),
    #[error("Service Error: {0}")]
    ServiceError(#[from] ServiceError),
    #[error("Service Error: {0}")]
    MailboxError(#[from] MailboxError),
    #[error("Other Error: {0}")]
    Other(String),
}

impl LibError {
    pub fn other(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}

unsafe impl Send for LibError {}
