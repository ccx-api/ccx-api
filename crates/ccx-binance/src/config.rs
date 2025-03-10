use url::Url;
use url_macro::url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
    pub(crate) combined_stream_base: Url,
    pub(crate) raw_stream_base: Url,
}

impl ConnectionConfig {
    pub fn new(api_base: Url, combined_stream_base: Url, raw_stream_base: Url) -> Self {
        ConnectionConfig {
            api_base,
            combined_stream_base,
            raw_stream_base,
        }
    }
}

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
