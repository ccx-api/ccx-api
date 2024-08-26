use std::io;

use actix::io::SinkWrite;
use actix::prelude::*;
use futures::channel::mpsc;
use url::Url;

use super::websocket::Websocket;
use crate::client::RestExchangeClient;
use crate::error::CoinbaseError;
use crate::error::CoinbaseResult;
use crate::proto::message::ClientMessage;
use crate::proto::subscribe::Subscribe;
use crate::proto::WsCommand;

pub struct WebsocketStream {
    addr: Addr<Websocket>,
    rx: mpsc::UnboundedReceiver<ClientMessage>,
}

impl WebsocketStream {
    pub async fn connect<S: crate::client::CoinbaseExchangeSigner>(
        api_client: RestExchangeClient<S>,
        url: Url,
    ) -> CoinbaseResult<Self> {
        use futures::StreamExt;

        log::debug!("Connecting WS: {}", url.as_str());

        let (response, connection) = api_client.client().ws(url.as_str()).connect().await?;
        log::debug!("{:?}", response);

        let (sink, stream) = connection.split();
        let (tx, rx) = mpsc::unbounded();
        let addr = Websocket::create(move |ctx| {
            Websocket::add_stream(stream, ctx);
            Websocket::new(SinkWrite::new(sink, ctx), tx)
        });

        Ok(WebsocketStream { addr, rx })
    }

    pub fn split(self) -> (Addr<Websocket>, mpsc::UnboundedReceiver<ClientMessage>) {
        (self.addr, self.rx)
    }

    pub async fn subscribe_one(&self, subscription: impl Into<Subscribe>) -> CoinbaseResult<()> {
        let cmd = WsCommand::Subscribe(subscription.into());
        self.addr
            .send(cmd)
            .await
            .map_err(|_e| CoinbaseError::IoError(io::ErrorKind::ConnectionAborted.into()))
    }
}
