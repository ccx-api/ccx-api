use std::{io, time};

use thiserror::Error;
// use reqwest;
use serde_json;
use url;
use awc::error::{SendRequestError, JsonPayloadError, WsClientError, WsProtocolError};
use awc::http::header::InvalidHeaderValue;

#[derive(Clone, Debug, Error)]
pub enum ServiceError {
    #[error("Server Error")]
    ServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
}

#[derive(Clone, Debug, Error)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
}

pub type LibResult<T> = std::result::Result<T, LibError>;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("Client Error: {0}")]
    ApiError(#[from] ApiError),
    #[error("Service Error: {0}")]
    ServiceError(#[from] ServiceError),
    #[error("Unknown Status: {0}")]
    UnknownStatus(awc::http::StatusCode),
    #[error("Request Error: {0}")]
    RequestError(#[from] SendRequestError),
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
    #[error("Json2 Error: {0}")]
    Json2(#[from] JsonPayloadError),
    #[error("Time Error: {0}")]
    TimestampError(#[from] time::SystemTimeError),
    #[error("Websocket Client Error: {0}")]
    WsClientError(#[from] WsClientError),
    #[error("Websocket Protocol Error: {0}")]
    WsProtocolError(#[from] WsProtocolError),
}
