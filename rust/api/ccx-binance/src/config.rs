use url::Url;

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
