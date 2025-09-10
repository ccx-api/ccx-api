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

        // Create awc client with HTTP/1.1 only to avoid SOCKS5 proxy HTTP/2 tunneling issues
        let mut tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates({
                let mut root_store = rustls::RootCertStore::empty();
                root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
                    |ta| {
                        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                            ta.subject,
                            ta.spki,
                            ta.name_constraints,
                        )
                    },
                ));
                root_store
            })
            .with_no_client_auth();

        // Force HTTP/1.1 only
        tls_config.alpn_protocols = vec![b"http/1.1".to_vec()];

        let connector = awc::Connector::new()
            .rustls(std::sync::Arc::new(tls_config))
            .timeout(std::time::Duration::from_secs(10));

        let awc_client = awc::Client::builder()
            .connector(connector)
            .timeout(std::time::Duration::from_secs(60))
            .finish();

        let (response, connection) = awc_client
            .ws(url.as_str())
            .connect()
            .await
            .map_err(|e| CoinbaseError::other(format!("WebSocket connection failed: {}", e)))?;
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
