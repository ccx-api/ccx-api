use url::Url;
use url_macro::url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
    pub(crate) raw_stream_base: Url,
}

impl ConnectionConfig {
    pub fn new(api_base: Url, raw_stream_base: Url) -> Self {
        ConnectionConfig {
            api_base,
            raw_stream_base,
        }
    }
}

pub fn production() -> ConnectionConfig {
    ConnectionConfig::new(
        url!("https://api.mexc.com/"),
        url!("wss://wbs-api.mexc.com/ws"),
    )
}
