use std::borrow::Cow;

use thiserror::Error;

pub use ccx_api_lib::*;

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

pub type BinanceResult<T> = ccx_api_lib::LibResult<T, ApiError>;
pub type BinanceError = ccx_api_lib::LibError<ApiError>;

fn test() {
    let y = x();

    // fn x() -> LibError<KrakenApiError> {
    fn x() -> BinanceResult<()> {
        todo!()
    }
}
