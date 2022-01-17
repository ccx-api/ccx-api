mod config;
#[cfg(feature = "with_proxy")]
mod connector;
mod rest;
mod signer;

pub use self::config::*;
pub use self::rest::*;
pub use self::signer::*;

pub const API_BASE: &str = "https://bpay.binanceapi.com/";
pub const API_BASE_TESTNET: &str = "https://bpay.binanceapi.com/";

#[derive(Debug, Deserialize)]
struct BinanceContentError {
    #[allow(dead_code)]
    pub code: i16,
    #[allow(dead_code)]
    pub msg: String,
}
