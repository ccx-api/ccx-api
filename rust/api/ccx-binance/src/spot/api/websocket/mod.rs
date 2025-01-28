mod multiplexed;
mod raw;
mod subscribe;

use smart_string::SmartString;

pub use self::raw::RawWebSocket;
use crate::spot::client::BinanceSpotClient;
use crate::spot::client::WebSocketConnectError;
use crate::spot::types::ws_events::Trade;
use crate::spot::types::ws_stream_name::StreamName;

pub struct WebSocketBuilder {
    client: BinanceSpotClient,
}

impl WebSocketBuilder {
    pub fn new(client: BinanceSpotClient) -> Self {
        WebSocketBuilder { client }
    }

    pub async fn raw_trade(
        &self,
        symbol: SmartString,
    ) -> Result<RawWebSocket<Trade>, WebSocketConnectError> {
        let stream_url = self.client.config().raw_stream_base.clone();
        let stream_name = StreamName::Trade { symbol };
        RawWebSocket::connect(stream_url, stream_name).await
    }
}
