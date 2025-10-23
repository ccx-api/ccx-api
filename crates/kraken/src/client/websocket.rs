use std::time::Duration;

use ccx_api_lib::Seq;
use futures::channel::mpsc;
use futures::StreamExt;
use futures::SinkExt;
use tokio::time::Instant;
use tokio::time::interval;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream as TungsteniteStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use crate::client::KrakenSigner;
use crate::client::RestClient;
use crate::error::KrakenError;
use crate::error::KrakenResult;
use crate::ws_stream::UpstreamApiRequest;
use crate::ws_stream::UpstreamWebsocketMessage;
use crate::ws_stream::WsCommand;
use crate::ws_stream::WsEvent;
use crate::ws_stream::WsSubscription;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebsocketStream {
    tx: WebsocketStreamTx,
    rx: mpsc::UnboundedReceiver<UpstreamWebsocketMessage<WsEvent>>,
}

#[derive(Clone)]
pub struct WebsocketStreamTx {
    command_tx: mpsc::UnboundedSender<WsCommand>,
}

impl WebsocketStream {
    pub async fn connect<S: KrakenSigner>(
        _api_client: RestClient<S>,
        url: Url,
    ) -> KrakenResult<Self> {
        log::debug!("Connecting WS: {}", url.as_str());

        let (ws_stream, response) = connect_async(url.as_str())
            .await
            .map_err(|e| KrakenError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        log::debug!("WebSocket connected: {:?}", response.status());

        let (command_tx, command_rx) = mpsc::unbounded();
        let (message_tx, message_rx) = mpsc::unbounded();

        // Spawn background task to handle WebSocket
        tokio::spawn(async move {
            if let Err(e) = run_websocket(ws_stream, command_rx, message_tx).await {
                log::error!("WebSocket error: {:?}", e);
            }
        });

        let tx = WebsocketStreamTx { command_tx };
        Ok(WebsocketStream { tx, rx: message_rx })
    }

    pub fn split(
        self,
    ) -> (
        WebsocketStreamTx,
        mpsc::UnboundedReceiver<UpstreamWebsocketMessage<WsEvent>>,
    ) {
        (self.tx, self.rx)
    }
}

impl std::ops::Deref for WebsocketStream {
    type Target = WebsocketStreamTx;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

impl WebsocketStreamTx {
    pub async fn subscribe(&self, subscription: impl Into<WsSubscription>) -> KrakenResult<()> {
        let cmd = WsCommand::Subscribe(subscription.into());
        self.command_tx
            .unbounded_send(cmd)
            .map_err(|_| KrakenError::IoError(std::io::ErrorKind::ConnectionAborted.into()))
    }
}

async fn run_websocket(
    ws_stream: TungsteniteStream<MaybeTlsStream<tokio::net::TcpStream>>,
    mut command_rx: mpsc::UnboundedReceiver<WsCommand>,
    message_tx: mpsc::UnboundedSender<UpstreamWebsocketMessage<WsEvent>>,
) -> KrakenResult<()> {
    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    let mut heartbeat_interval = interval(HEARTBEAT_INTERVAL);
    let mut last_heartbeat = Instant::now();
    let mut id_seq = Seq::new();

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<UpstreamWebsocketMessage<WsEvent>>(&text) {
                            Ok(UpstreamWebsocketMessage::Event(
                                WsEvent::Pong(_) | WsEvent::Heartbeat(_),
                            )) => {
                                last_heartbeat = Instant::now();
                            }
                            Ok(message) => {
                                if let Err(e) = message_tx.unbounded_send(message) {
                                    log::warn!("Failed to send message to receiver: {:?}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to deserialize server message: {:?}. Message: {}",
                                    e,
                                    text
                                );
                            }
                        }
                    }
                    Some(Ok(Message::Binary(_))) => {
                        log::warn!("Unexpected binary message (ignored)");
                    }
                    Some(Ok(Message::Ping(data))) => {
                        last_heartbeat = Instant::now();
                        if let Err(e) = ws_sink.send(Message::Pong(data)).await {
                            log::warn!("Failed to send pong: {:?}", e);
                            break;
                        }
                    }
                    Some(Ok(Message::Pong(_))) => {
                        last_heartbeat = Instant::now();
                    }
                    Some(Ok(Message::Close(_))) => {
                        log::info!("WebSocket closed by server");
                        break;
                    }
                    Some(Ok(Message::Frame(_))) => {
                        // Handled automatically
                    }
                    Some(Err(e)) => {
                        log::error!("WebSocket error: {:?}", e);
                        break;
                    }
                    None => {
                        log::info!("WebSocket connection closed");
                        break;
                    }
                }
            }

            // Handle outgoing commands
            cmd = command_rx.next() => {
                match cmd {
                    Some(command) => {
                        let msg = UpstreamApiRequest {
                            reqid: id_seq.next(),
                            payload: command,
                        };
                        let msg_str = serde_json::to_string(&msg).expect("json encode");
                        log::debug!("Sending to server: `{}`", msg_str);
                        if let Err(e) = ws_sink.send(Message::Text(msg_str.into())).await {
                            log::error!("Failed to send message: {:?}", e);
                            break;
                        }
                    }
                    None => {
                        log::info!("Command channel closed, shutting down");
                        break;
                    }
                }
            }

            // Handle heartbeat
            _ = heartbeat_interval.tick() => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::warn!("WebSocket client heartbeat timeout, disconnecting!");
                    break;
                }

                if let Err(e) = ws_sink.send(Message::Ping(vec![].into())).await {
                    log::warn!("Failed to send ping: {:?}", e);
                    break;
                }
            }
        }
    }

    // Clean shutdown
    let _ = ws_sink.close().await;
    Ok(())
}
