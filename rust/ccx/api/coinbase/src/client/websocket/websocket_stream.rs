use std::io;

use actix::prelude::*;
use actix_http::Version;
use futures::channel::mpsc;

use super::websocket::Websocket;
use crate::client::websocket::websocket::ReconnectSocket;
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
    pub fn connect(subscription: impl Into<Subscribe>) -> CoinbaseResult<Self> {
        let (tx, rx) = mpsc::unbounded();

        let client = awc::Client::builder()
            .max_http_version(Version::HTTP_11)
            .finish();

        // wss://ws-direct.exchange.coinbase.com
        let url = "wss://ws-feed.exchange.coinbase.com".try_into().unwrap();
        let addr = Websocket::new(client, url, tx).start();
        addr.try_send(ReconnectSocket)
            .map_err(|_| CoinbaseError::IoError(io::ErrorKind::ConnectionAborted.into()))?;

        let cmd = WsCommand::Subscribe(subscription.into());
        addr.try_send(cmd)
            .map_err(|_| CoinbaseError::IoError(io::ErrorKind::ConnectionAborted.into()))?;

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
