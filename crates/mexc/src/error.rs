use std::borrow::Cow;

pub use ccx_api_lib::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Mandatory field(s) omitted: {0}")]
    MandatoryFieldOmitted(Cow<'static, str>),
    #[error("Argument is out of bounds")]
    OutOfBounds,
}

impl ApiError {
    pub fn mandatory_field_omitted(field: impl Into<Cow<'static, str>>) -> Self {
        ApiError::MandatoryFieldOmitted(field.into())
    }
}

impl CcxApiError for ApiError {}

pub type MexcResult<T> = ccx_api_lib::LibResult<T, ApiError>;
pub type MexcError = ccx_api_lib::LibError<ApiError>;
