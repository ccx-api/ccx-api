use std::time::Duration;

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

use crate::client::RestClient;
use crate::error::GateError;
use crate::error::GateResult;
use crate::websocket::request::WsRequest;
use crate::websocket::request::WsRequestEvent;
use crate::websocket::order_book::OrderBookRequest;
use crate::websocket::response::Event;
use crate::websocket::response::WsResponse;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebsocketStream {
    tx: WebsocketStreamTx,
    rx: mpsc::UnboundedReceiver<WsResponse>,
}

#[derive(Clone)]
pub struct WebsocketStreamTx {
    command_tx: mpsc::UnboundedSender<WsRequest>,
}

impl WebsocketStream {
    pub async fn connect<S>(
        _api_client: RestClient<S>,
        url: Url,
    ) -> GateResult<Self> {
        log::debug!("Connecting WS: {}", url.as_str());

        let (ws_stream, response) = connect_async(url.as_str())
            .await
            .map_err(|e| GateError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

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

    pub fn split(self) -> (WebsocketStreamTx, mpsc::UnboundedReceiver<WsResponse>) {
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
    pub async fn send(&self, request: WsRequest) -> GateResult<()> {
        self.command_tx
            .unbounded_send(request)
            .map_err(|_| GateError::IoError(std::io::ErrorKind::ConnectionAborted.into()))
    }

    /// Subscribe or unsubscribe from order book snapshots
    pub async fn order_book(
        &self,
        event: WsRequestEvent,
        payload: OrderBookRequest,
    ) -> GateResult<()> {
        self.send(WsRequest::order_book(event, payload)).await
    }
}

async fn run_websocket(
    ws_stream: TungsteniteStream<MaybeTlsStream<tokio::net::TcpStream>>,
    mut command_rx: mpsc::UnboundedReceiver<WsRequest>,
    message_tx: mpsc::UnboundedSender<WsResponse>,
) -> GateResult<()> {
    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    let mut heartbeat_interval = interval(HEARTBEAT_INTERVAL);
    let mut last_heartbeat = Instant::now();

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<WsResponse>(&text) {
                            Ok(WsResponse {
                                event: Event::Pong(Ok(())),
                                ..
                            }) => {
                                last_heartbeat = Instant::now();
                            }
                            Ok(response) => {
                                if let Err(e) = message_tx.unbounded_send(response) {
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
                    Some(request) => {
                        let msg = serde_json::to_string(&request).expect("json encode");
                        log::debug!("Sending to server: `{}`", msg);
                        if let Err(e) = ws_sink.send(Message::Text(msg.into())).await {
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
