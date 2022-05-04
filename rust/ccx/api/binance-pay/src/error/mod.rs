use std::borrow::Cow;
use std::fmt;
use std::string::FromUtf8Error;

use futures::channel::oneshot::Canceled;
use thiserror::Error;

use ccx_api_lib::SignError;

pub mod common_error;

#[cfg(feature = "with_network")]
mod with_network {
    pub use awc::error::JsonPayloadError;
    pub use awc::error::PayloadError;
    pub use awc::error::SendRequestError;
    pub use awc::error::WsClientError;
    pub use awc::error::WsProtocolError;
    pub use awc::http::header::InvalidHeaderValue;
}

#[cfg(feature = "with_network")]
use self::with_network::*;

#[derive(Clone, Debug, Error)]
pub enum ServiceError {
    #[error("Server Error")]
    ServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
}

#[derive(Clone, Debug, Error)]
pub enum RequestError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Mandatory field(s) omitted: {0}")]
    MandatoryFieldOmitted(Cow<'static, str>),
    #[error("Argument is out of bounds")]
    OutOfBounds,
}

impl RequestError {
    pub fn mandatory_field_omitted(field: impl Into<Cow<'static, str>>) -> Self {
        RequestError::MandatoryFieldOmitted(field.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub struct BinanceError {
    code: i32,
    msg: String,
    params: Option<Vec<String>>,
}

impl fmt::Display for BinanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BinanceError {{ code: {}; reason: {}",
            self.code, self.msg
        )?;
        if let Some(params) = &self.params {
            write!(f, "; params: {:?}", params)?;
        }
        write!(f, " }}")
    }
}

pub type LibResult<T> = std::result::Result<T, LibError>;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("Binance Error: {0}")]
    BinanceError(#[from] BinanceError),
    #[error("Client Error: {0}")]
    ApiError(#[from] RequestError),
    #[error("Service Error: {0}")]
    ServiceError(#[from] ServiceError),
    #[error("Unknown Status: {0}")]
    UnknownStatus(awc::http::StatusCode),
    #[error("Request Error: {0}")]
    RequestError(#[from] SendRequestError),
    #[error("Payload Error: {0}")]
    Payload(#[from] PayloadError),
    #[error("Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Url Encoded Error: {0}")]
    UrlEncodedError(#[from] serde_urlencoded::ser::Error),
    #[error("Invalid Header: {0}")]
    InvalidHeaderError(#[from] InvalidHeaderValue),
    #[error("Url Parse Error: {0}")]
    UrlParserError(#[from] url::ParseError),
    #[error("Url Parse Error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("Channel Error: {0}")]
    ChannelError(#[from] Canceled),
    #[error("Sign Error: {0}")]
    SignError(#[from] SignError),
    #[error("Other Error: {0}")]
    Other(String),
}

impl LibError {
    pub fn other(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}
