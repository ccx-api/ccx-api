use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Once;
use std::time::Duration;

use bytes::Bytes;
use futures::channel::mpsc;
use futures::prelude::*;
use string_cache::DefaultAtom as Atom;
use tokio::sync::Mutex;
use tokio::time::Instant;
use tokio::time::interval;
use tokio::time::timeout;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use url::Url;

use crate::error::CoinbaseError;
use crate::error::CoinbaseResult;
use crate::proto::WsCommand;
use crate::proto::message::ClientMessage;
use crate::proto::subscribe::ChannelType;
use crate::proto::subscribe::Subscribe;
use crate::proto::subscribe::Unsubscribe;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

/// WebSocket connection configuration
#[derive(Debug, Clone)]
pub struct WebsocketConfig {
    pub heartbeat_interval: Duration,
    pub client_timeout: Duration,
    pub connect_timeout: Duration,
}

impl Default for WebsocketConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: HEARTBEAT_INTERVAL,
            client_timeout: CLIENT_TIMEOUT,
            connect_timeout: Duration::from_secs(10),
        }
    }
}

/// Modern WebSocket client using tokio-tungstenite
pub struct Websocket {
    /// WebSocket stream
    ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    /// Channel sender for outgoing messages
    message_tx: mpsc::UnboundedSender<ClientMessage>,
    /// Subscribed channels tracking
    channels: Arc<Mutex<HashMap<ChannelType, HashSet<Atom>>>>,
    /// Configuration
    config: WebsocketConfig,
    /// Last heartbeat time
    last_heartbeat: Arc<Mutex<Instant>>,
}

impl Websocket {
    /// Create a new WebSocket connection
    pub async fn connect(
        url: Url,
        config: Option<WebsocketConfig>,
    ) -> CoinbaseResult<(Self, mpsc::UnboundedReceiver<ClientMessage>)> {
        let config = config.unwrap_or_default();

        log::debug!("Connecting WS: {}", url.as_str());

        // Use default TLS connector for now
        let connector: Option<tokio_tungstenite::Connector> = None;

        // Set WebSocket config
        let ws_config = WebSocketConfig::default()
            .read_buffer_size(256 * 1024)
            .write_buffer_size(256 * 1024);

        // Connect with timeout
        let (ws_stream, response) = timeout(
            config.connect_timeout,
            connect_async_with_config(url.as_str(), Some(ws_config), false),
        )
        .await
        .map_err(|_| CoinbaseError::other("WebSocket connection timeout"))?
        .map_err(|e| CoinbaseError::other(format!("WebSocket connection failed: {}", e)))?;

        log::debug!("WebSocket connected: {:?}", response.status());

        let (message_tx, message_rx) = mpsc::unbounded();
        let channels = Arc::new(Mutex::new(HashMap::new()));
        let last_heartbeat = Arc::new(Mutex::new(Instant::now()));

        let websocket = Self {
            ws_stream,
            message_tx,
            channels,
            config,
            last_heartbeat,
        };

        Ok((websocket, message_rx))
    }

    /// Run the WebSocket client event loop with command handling
    pub async fn run_with_commands(
        mut self,
        mut command_rx: mpsc::UnboundedReceiver<WsCommand>,
    ) -> CoinbaseResult<()> {
        let mut heartbeat_interval = interval(self.config.heartbeat_interval);
        let message_tx = self.message_tx.clone();
        let last_heartbeat = Arc::clone(&self.last_heartbeat);
        let client_timeout = self.config.client_timeout;

        loop {
            tokio::select! {
                // Handle incoming WebSocket messages
                msg = self.ws_stream.next() => {
                    match msg {
                        Some(Ok(message)) => {
                            if let Err(e) = self.handle_message(message).await {
                                log::error!("Error handling WebSocket message: {:?}", e);
                                break;
                            }
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

                // Handle incoming commands from user
                cmd = command_rx.next() => {
                    match cmd {
                        Some(command) => {
                            if let Err(e) = self.send_command(command).await {
                                log::error!("Failed to send WebSocket command: {:?}", e);
                                break;
                            }
                        }
                        None => {
                            log::info!("Command channel closed, shutting down WebSocket");
                            break;
                        }
                    }
                }

                // Handle heartbeat
                _ = heartbeat_interval.tick() => {
                    let last_hb = *last_heartbeat.lock().await;
                    if Instant::now().duration_since(last_hb) > client_timeout {
                        log::warn!("WebSocket client heartbeat timeout, disconnecting!");
                        break;
                    }

                    if let Err(e) = self.ws_stream.send(Message::Ping(Bytes::new())).await {
                        log::warn!("Failed to send ping: {:?}", e);
                        break;
                    }
                }
            }
        }

        // Clean shutdown
        let _ = self.ws_stream.close(None).await;
        Ok(())
    }

    /// Handle incoming WebSocket message
    async fn handle_message(&mut self, message: Message) -> CoinbaseResult<()> {
        match message {
            Message::Text(text) => {
                self.handle_text_message(text.as_bytes()).await?;
            }
            Message::Binary(data) => {
                log::warn!("Unexpected binary message (ignored)");
            }
            Message::Ping(data) => {
                *self.last_heartbeat.lock().await = Instant::now();
                if let Err(e) = self.ws_stream.send(Message::Pong(data)).await {
                    log::warn!("Failed to send pong: {:?}", e);
                }
            }
            Message::Pong(_) => {
                *self.last_heartbeat.lock().await = Instant::now();
            }
            Message::Close(_) => {
                log::info!("Received close frame");
                return Err(CoinbaseError::other("WebSocket closed by server"));
            }
            Message::Frame(_frame) => {
                // tokio-tungstenite handles frame assembly automatically
                log::debug!("Received raw frame (handled automatically)");
            }
        }
        Ok(())
    }

    /// Handle text message
    async fn handle_text_message(&mut self, data: &[u8]) -> CoinbaseResult<()> {
        let message = match serde_json::from_slice::<ClientMessage>(data) {
            Ok(msg) => msg,
            Err(e) => {
                log::error!(
                    "Failed to deserialize server message: {:?}, data: {}",
                    e,
                    String::from_utf8_lossy(data)
                );
                return Ok(()); // Continue processing other messages
            }
        };

        if let Err(e) = self.message_tx.unbounded_send(message) {
            log::warn!("Failed to send message to receiver: {:?}", e);
            return Err(CoinbaseError::other("Message channel closed"));
        }

        Ok(())
    }

    /// Send a command to the WebSocket server
    pub async fn send_command(&mut self, cmd: WsCommand) -> CoinbaseResult<()> {
        let msg = serde_json::to_string(&cmd)
            .map_err(|e| CoinbaseError::other(format!("Failed to serialize command: {}", e)))?;

        log::debug!("Sending to server: `{}`", msg);

        self.ws_stream
            .send(Message::Text(msg.into()))
            .await
            .map_err(|e| {
                CoinbaseError::other(format!("Failed to send WebSocket message: {}", e))
            })?;

        // Update channel subscriptions
        self.update_subscriptions(&cmd).await;

        Ok(())
    }

    /// Update internal subscription tracking
    async fn update_subscriptions(&mut self, cmd: &WsCommand) {
        let mut channels = self.channels.lock().await;

        match cmd {
            WsCommand::Subscribe(Subscribe {
                product_ids,
                channels: sub_channels,
            }) => {
                for channel in sub_channels {
                    let entry = channels.entry(channel.clone()).or_default();
                    for product_id in product_ids {
                        entry.insert(product_id.clone());
                    }
                }
            }
            WsCommand::Unsubscribe(Unsubscribe {
                product_ids,
                channels: unsub_channels,
            }) => {
                for channel in unsub_channels {
                    if let Some(set) = channels.get_mut(channel) {
                        for product_id in product_ids {
                            set.remove(product_id);
                        }
                    }
                }
            }
        }
    }

    /// Get current subscriptions
    pub async fn get_subscriptions(&self) -> HashMap<ChannelType, HashSet<Atom>> {
        self.channels.lock().await.clone()
    }

    /// Close the WebSocket connection
    pub async fn close(mut self) -> CoinbaseResult<()> {
        self.ws_stream
            .close(None)
            .await
            .map_err(|e| CoinbaseError::other(format!("Failed to close WebSocket: {}", e)))?;
        Ok(())
    }
}
