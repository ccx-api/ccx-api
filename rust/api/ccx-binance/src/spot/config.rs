use url_macro::url;

use crate::config::ConnectionConfig;

pub fn production() -> ConnectionConfig {
    ConnectionConfig {
        api_base: url!("https://api.binance.com/"),
        stream_base: url!("wss://stream.binance.com/stream"),
    }
}

pub fn sandbox() -> ConnectionConfig {
    ConnectionConfig {
        api_base: url!("https://testnet.binance.vision/"),
        stream_base: url!("wss://testnet.binance.vision/stream"),
    }
}
