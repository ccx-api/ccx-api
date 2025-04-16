use crate::api::error::KrakenApiError;
pub use crate::client::meta::{KrakenErrorWithMeta, KrakenResponseWithMeta};

impl ccx_lib::CcxApiError for KrakenApiError {}

pub type KrakenResult<T> = Result<KrakenResponseWithMeta<T>, KrakenErrorWithMeta>;
