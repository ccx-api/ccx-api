use url_macro::url;

use url::Url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
}

impl ConnectionConfig {
    /// Use new to specify address of BitGo Express server
    pub fn new(api_base: Url) -> Self {
        ConnectionConfig { api_base }
    }
}

pub fn production() -> ConnectionConfig {
    ConnectionConfig::new(url!("https://app.bitgo.com/"))
}

pub fn testing() -> ConnectionConfig {
    ConnectionConfig::new(url!("https://app.bitgo-test.com/"))
}
