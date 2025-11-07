use url_macro::url;

use url::Url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
    pub(crate) websocket_base: Url,
}

impl ConnectionConfig {
    pub fn new(api_base: Url, websocket_base: Url) -> Self {
        ConnectionConfig {
            api_base,
            websocket_base,
        }
    }
}

pub fn production() -> ConnectionConfig {
    ConnectionConfig::new(
        url!("https://api.gateio.ws"),
        url!("wss://api.gateio.ws/ws/v4/"),
    )
}

pub fn testing() -> ConnectionConfig {
    ConnectionConfig::new(
        url!("https://api-testnet.gateapi.io/api/v4"),
        url!("wss://ws-testnet.gate.com/v4/ws/spot/"),
    )
}
