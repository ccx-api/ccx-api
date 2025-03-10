use crate::api::error::GateApiError;
use crate::client::meta::{GateErrorWithMeta, GateResponseWithMeta};

impl ccx_lib::CcxApiError for GateApiError {}

pub type GateResult<T> = Result<GateResponseWithMeta<T>, GateErrorWithMeta>;
