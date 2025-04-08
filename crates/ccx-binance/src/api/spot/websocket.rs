mod raw;

use ccx_lib::websocket::WebSocketConnectError;
use smart_string::SmartString;

pub use self::raw::RawWebSocket;
use crate::client::BinanceClient;
use crate::types::ws_events::DepthUpdateEvent;
use crate::types::ws_events::TradeEvent;
use crate::types::ws_stream_name::DepthUpdateSpeed;
use crate::types::ws_stream_name::StreamName;

pub struct WebSocketBuilder {
    client: BinanceClient,
}

impl WebSocketBuilder {
    pub fn new(client: BinanceClient) -> Self {
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
        symbol: &str,
        update_speed: DepthUpdateSpeed,
    ) -> Result<RawWebSocket<DepthUpdateEvent>, WebSocketConnectError> {
        let stream_url = self.client.config().raw_stream_base.clone();
        let stream_name = StreamName::BookDepth {
            symbol: symbol.to_lowercase().into(),
            // TODO: with levels the output format is too different to without level
            // and that should be covered by another method probably
            levels: None,
            update_speed,
        };
        RawWebSocket::connect(stream_url, stream_name).await
    }
}
