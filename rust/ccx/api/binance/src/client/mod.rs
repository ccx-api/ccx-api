mod config;
mod rest;
mod websocket;
mod websocket2;

pub use self::config::*;
pub use self::rest::*;
pub use self::websocket::*;
pub use self::websocket2::*;

/// The base enpoint.
pub const API_BASE: &str = "https://api.binance.com/";
pub const STREAM_BASE: &str = "wss://stream.binance.com/stream";

#[derive(Debug, Deserialize)]
struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}
