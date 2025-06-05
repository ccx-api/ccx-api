use crate::api::error::BitGoApiError;
pub use crate::client::meta::{BitGoErrorWithMeta, BitGoResponseWithMeta};

impl ccx_lib::CcxApiError for BitGoApiError {}

pub type BitGoResult<T> = Result<BitGoResponseWithMeta<T>, BitGoErrorWithMeta>;
