use serde::Deserialize;
use serde::Serialize;

use crate::client::meta::BinanceResponseMeta;

#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
#[error("Binance Error: {code} - {msg}")]
pub struct BinanceApiError {
    pub code: i32,
    pub msg: String,
}

impl ccx_lib::CcxApiError for BinanceApiError {}

pub type BinanceError = ccx_lib::Error<BinanceApiError>;

#[derive(Debug, thiserror::Error)]
#[error("Binance Request Error: {error}")]
pub struct BinanceErrorWithMeta {
    pub error: BinanceError,
    pub meta: Option<BinanceResponseMeta>,
}

impl BinanceErrorWithMeta {
    pub fn new(error: BinanceError, meta: Option<BinanceResponseMeta>) -> Self {
        BinanceErrorWithMeta { error, meta }
    }
}

impl<T: Into<BinanceError>> From<T> for BinanceErrorWithMeta {
    fn from(error: T) -> Self {
        BinanceErrorWithMeta {
            error: error.into(),
            meta: None,
        }
    }
}
