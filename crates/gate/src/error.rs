pub use ccx_api_lib::*;

use crate::api::GateApiError;

pub type GateResult<T> = ccx_api_lib::LibResult<T, GateApiError>;
pub type GateError = ccx_api_lib::LibError<GateApiError>;

impl CcxApiError for GateApiError {}
