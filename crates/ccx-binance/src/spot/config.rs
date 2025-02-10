use url_macro::url;

use crate::config::ConnectionConfig;

pub fn production() -> ConnectionConfig {
    ConnectionConfig::new(
        url!("https://api.binance.com/"),
        url!("wss://stream.binance.com/stream"),
        url!("wss://stream.binance.com/ws/"),
    )
}

pub fn sandbox() -> ConnectionConfig {
    ConnectionConfig::new(
        url!("https://testnet.binance.vision/"),
        url!("wss://testnet.binance.vision/stream"),
        url!("wss://testnet.binance.vision/ws/"),
    )
}
