use url::Url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
    pub(crate) stream_base: Url,
}

impl ConnectionConfig {
    pub fn new(api_base: Url, stream_base: Url) -> Self {
        ConnectionConfig {
            api_base,
            stream_base,
        }
    }
}
