use std::borrow::Cow;
use std::fmt;

pub use ccx_api_lib::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ApiErrorKind {
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
    #[error("Unauthorized")]
    Unauthorized,
    // #[error("Mandatory field(s) omitted: {0}")]
    // MandatoryFieldOmitted(Cow<'static, str>),
    // #[error("Argument is out of bounds")]
    // OutOfBounds,
    #[error("Unrecognized")]
    Unrecognized,
    #[error("Unknown")]
    Unknown(Cow<'static, str>),
}

#[derive(Debug, Error)]
pub struct BitstampApiError(pub ApiErrorKind, pub StatusCode, pub String);

impl ApiErrorKind {
    pub fn unknown(msg: impl Into<Cow<'static, str>>) -> Self {
        ApiErrorKind::Unknown(msg.into())
    }

    pub fn from_string(s: String) -> Self {
        // match () {
        //     () if s.starts_with("EGeneral:Invalid arguments") => Self::InvalidArguments,
        //     () if s.starts_with("EGeneral:Internal error") => Self::InternalError,
        //     () if s.starts_with("EGeneral:Permission denied") => Self::PermissionDenied,
        //     () if s.starts_with("EAPI:Invalid key") => Self::InvalidKey,
        //     _ => Self::unknown(s),
        // }
        Self::unknown(s)
    }
}

impl BitstampApiError {
    pub fn ok<T>(v: T) -> BitstampApiResult<T> {
        Ok(v)
    }

    pub fn lib_error(msg: &dyn fmt::Display) -> Self {
        BitstampApiError(
            ApiErrorKind::InternalError,
            StatusCode::BAD_REQUEST,
            format!("LibError:{msg}"),
        )
    }
}

pub type BitstampResult<T> = ccx_api_lib::LibResult<T, BitstampApiError>;
pub type BitstampApiResult<T> = ccx_api_lib::LibResult<T, BitstampApiError>;
pub type BitstampError = ccx_api_lib::LibError<BitstampApiError>;

impl CcxApiError for ApiErrorKind {}
impl CcxApiError for BitstampApiError {}

impl fmt::Display for BitstampApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BitstampApiError")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl From<(ApiErrorKind, StatusCode, String)> for BitstampApiError {
    fn from((kind, code, message): (ApiErrorKind, StatusCode, String)) -> Self {
        BitstampApiError(kind, code, message)
    }
}

// impl From<(ApiErrorKind, StatusCode, String)> for LibError<BitstampApiError> {
//     fn from(error: (ApiErrorKind, StatusCode, String)) -> Self {
//         BitstampApiError::from(error).into()
//     }
// }
