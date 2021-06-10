mod config;
mod limits;
mod rest;
mod websocket;
mod websocket2;
use serde::Deserialize;

pub use self::config::*;
pub use self::limits::*;
pub use self::rest::*;
pub use self::websocket::*;
pub use self::websocket2::*;

#[derive(Debug, Deserialize)]
struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}
