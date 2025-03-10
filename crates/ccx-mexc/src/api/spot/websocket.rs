mod raw;

use ccx_lib::websocket::WebSocketConnectError;
use smart_string::SmartString;

pub use self::raw::RawWebSocket;
use crate::client::MexcClient;
use crate::types::ws_events::DepthUpdateEvent;
use crate::types::ws_events::TradeEvent;
use crate::types::ws_stream_name::StreamName;

pub struct WebSocketBuilder {
    client: MexcClient,
}

impl WebSocketBuilder {
    pub fn new(client: MexcClient) -> Self {
        WebSocketBuilder { client }
    }

    pub async fn raw_trade(
        &self,
        symbol: SmartString,
    ) -> Result<RawWebSocket<TradeEvent>, WebSocketConnectError> {
        let stream_url = self.client.config().raw_stream_base.clone();
        let stream_name = StreamName::Trade { symbol };
        RawWebSocket::connect(stream_url, stream_name).await
    }

    pub async fn raw_depth_update(
        &self,
        symbol: SmartString,
        level: u16,
    ) -> Result<RawWebSocket<DepthUpdateEvent>, WebSocketConnectError> {
        let stream_url = self.client.config().raw_stream_base.clone();
        let stream_name = StreamName::BookDepth { symbol, level };
        RawWebSocket::connect(stream_url, stream_name).await
    }
}
