use std::borrow::Cow;
use std::fmt;

use thiserror::Error;

pub use ccx_api_lib::*;

#[derive(Clone, Debug, Error)]
pub enum ApiError {
    #[error("Not signed")]
    NotSigned,
    #[error("Invalid arguments")]
    InvalidArguments,
    #[error("Invalid API key")]
    InvalidKey,
    #[error("Internal error")]
    InternalError,
    #[error("Permission denied")]
    PermissionDenied,
    // #[error("Unauthorized")]
    // Unauthorized,
    // #[error("Mandatory field(s) omitted: {0}")]
    // MandatoryFieldOmitted(Cow<'static, str>),
    // #[error("Argument is out of bounds")]
    // OutOfBounds,
    #[error("Unrecognized")]
    Unrecognized(Cow<'static, str>),
}

#[derive(Default, Debug, Error)]
pub struct KrakenApiError(pub Vec<ApiError>);

impl ApiError {
    pub fn unrecognized(msg: impl Into<Cow<'static, str>>) -> Self {
        ApiError::Unrecognized(msg.into())
    }

    pub fn from_string(s: String) -> Self {
        match () {
            () if s.starts_with("EGeneral:Invalid arguments") => Self::InvalidArguments,
            () if s.starts_with("EGeneral:Internal error") => Self::InternalError,
            () if s.starts_with("EGeneral:Permission denied") => Self::PermissionDenied,
            () if s.starts_with("EAPI:Invalid key") => Self::InvalidKey,
            () => Self::unrecognized(s),
        }
    }
}

impl KrakenApiError {
    pub fn ok<T>(v: T) -> KrakenApiResult<T> {
        Ok((v, KrakenApiError(vec![])))
    }
}

pub type KrakenResult<T> = ccx_api_lib::LibResult<T, KrakenApiError>;
pub type KrakenApiResult<T> = ccx_api_lib::LibResult<(T, KrakenApiError), KrakenApiError>;
pub type KrakenError = ccx_api_lib::LibError<KrakenApiError>;

impl CcxApiError for ApiError {}
impl CcxApiError for KrakenApiError {}

impl fmt::Display for KrakenApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.0 {
            i.fmt(f)?;
        }
        Ok(())
    }
}

impl From<ApiError> for KrakenApiError {
    fn from(error: ApiError) -> Self {
        KrakenApiError(vec![error])
    }
}

impl From<ApiError> for LibError<KrakenApiError> {
    fn from(error: ApiError) -> Self {
        KrakenApiError::from(error).into()
    }
}
