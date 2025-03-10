use serde::Deserialize;
use serde::Serialize;

use crate::client::meta::MexcResponseMeta;

#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
#[error("Mexc Error: {code} - {msg}")]
pub struct MexcApiError {
    pub code: i32,
    pub msg: String,
}

impl ccx_lib::CcxApiError for MexcApiError {}

pub type MexcError = ccx_lib::Error<MexcApiError>;

#[derive(Debug, thiserror::Error)]
#[error("Mexc Request Error: {error}")]
pub struct MexcErrorWithMeta {
    pub error: MexcError,
    pub meta: Option<MexcResponseMeta>,
}

impl MexcErrorWithMeta {
    pub fn new(error: MexcError, meta: Option<MexcResponseMeta>) -> Self {
        MexcErrorWithMeta { error, meta }
    }
}

impl<T: Into<MexcError>> From<T> for MexcErrorWithMeta {
    fn from(error: T) -> Self {
        MexcErrorWithMeta {
            error: error.into(),
            meta: None,
        }
    }
}
