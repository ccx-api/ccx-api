use std::sync::Arc;

use futures::StreamExt;
use futures::channel::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use url::Url;

use super::websocket::Websocket;
use super::websocket::WebsocketConfig;
use crate::client::RestExchangeClient;
use crate::error::CoinbaseError;
use crate::error::CoinbaseResult;
use crate::proto::WsCommand;
use crate::proto::message::ClientMessage;
use crate::proto::subscribe::Subscribe;

/// High-level WebSocket stream wrapper
pub struct WebsocketStream {
    /// WebSocket client handle
    ws_handle: Arc<Mutex<Option<WebsocketClient>>>,
    /// Message receiver
    message_rx: mpsc::UnboundedReceiver<ClientMessage>,
    /// Background task handle
    task_handle: JoinHandle<CoinbaseResult<()>>,
}

/// Internal WebSocket client wrapper
struct WebsocketClient {
    /// Command sender to WebSocket
    command_tx: mpsc::UnboundedSender<WsCommand>,
}

impl WebsocketStream {
    /// Connect to WebSocket server
    pub async fn connect<S: crate::client::CoinbaseExchangeSigner>(
        _api_client: RestExchangeClient<S>, // Keep for compatibility, might be used for auth later
        url: Url,
    ) -> CoinbaseResult<Self> {
        Self::connect_with_config(url, None).await
    }

    /// Connect to WebSocket server with custom configuration
    pub async fn connect_with_config(
        url: Url,
        config: Option<WebsocketConfig>,
    ) -> CoinbaseResult<Self> {
        log::debug!("Connecting WebSocket stream: {}", url.as_str());

        let (websocket, message_rx) = Websocket::connect(url, config).await?;
        let (command_tx, mut command_rx) = mpsc::unbounded::<WsCommand>();

        let ws_handle = Arc::new(Mutex::new(Some(WebsocketClient { command_tx })));
        let ws_handle_clone = Arc::clone(&ws_handle);

        // Spawn background task to handle WebSocket communication
        let task_handle = tokio::spawn(async move {
            // Run the WebSocket client with command handling integrated
            if let Err(e) = websocket.run_with_commands(command_rx).await {
                log::error!("WebSocket client error: {:?}", e);
                return Err(e);
            }

            // Clean up
            let mut handle = ws_handle_clone.lock().await;
            *handle = None;

            Ok(())
        });

        Ok(WebsocketStream {
            ws_handle,
            message_rx,
            task_handle,
        })
    }

    /// Split into sender handle and message receiver
    pub fn split(self) -> (WebsocketHandle, mpsc::UnboundedReceiver<ClientMessage>) {
        let handle = WebsocketHandle {
            ws_handle: Arc::clone(&self.ws_handle),
            task_handle: Some(self.task_handle),
        };
        (handle, self.message_rx)
    }

    /// Subscribe to channels (convenience method)
    pub async fn subscribe_one(&self, subscription: impl Into<Subscribe>) -> CoinbaseResult<()> {
        let cmd = WsCommand::Subscribe(subscription.into());
        self.send_command(cmd).await
    }

    /// Send a command to the WebSocket
    async fn send_command(&self, cmd: WsCommand) -> CoinbaseResult<()> {
        let handle = self.ws_handle.lock().await;
        if let Some(client) = handle.as_ref() {
            client
                .command_tx
                .unbounded_send(cmd)
                .map_err(|_| CoinbaseError::other("WebSocket command channel closed"))?;
            Ok(())
        } else {
            Err(CoinbaseError::other("WebSocket connection is closed"))
        }
    }

    /// Close the WebSocket connection
    pub async fn close(self) -> CoinbaseResult<()> {
        // Close the command channel
        {
            let mut handle = self.ws_handle.lock().await;
            *handle = None;
        }

        // Wait for background task to finish
        match self.task_handle.await {
            Ok(result) => result,
            Err(e) => {
                log::error!("WebSocket task join error: {:?}", e);
                Err(CoinbaseError::other("Failed to join WebSocket task"))
            }
        }
    }
}

/// Handle for sending commands to WebSocket
pub struct WebsocketHandle {
    ws_handle: Arc<Mutex<Option<WebsocketClient>>>,
    task_handle: Option<JoinHandle<CoinbaseResult<()>>>,
}

impl WebsocketHandle {
    /// Send a command to the WebSocket
    pub async fn send_command(&self, cmd: WsCommand) -> CoinbaseResult<()> {
        let handle = self.ws_handle.lock().await;
        if let Some(client) = handle.as_ref() {
            client
                .command_tx
                .unbounded_send(cmd)
                .map_err(|_| CoinbaseError::other("WebSocket command channel closed"))?;
            Ok(())
        } else {
            Err(CoinbaseError::other("WebSocket connection is closed"))
        }
    }

    /// Subscribe to channels
    pub async fn subscribe(&self, subscription: impl Into<Subscribe>) -> CoinbaseResult<()> {
        let cmd = WsCommand::Subscribe(subscription.into());
        self.send_command(cmd).await
    }

    /// Unsubscribe from channels
    pub async fn unsubscribe(
        &self,
        unsubscription: impl Into<crate::proto::subscribe::Unsubscribe>,
    ) -> CoinbaseResult<()> {
        let cmd = WsCommand::Unsubscribe(unsubscription.into());
        self.send_command(cmd).await
    }

    /// Check if WebSocket is still connected
    pub async fn is_connected(&self) -> bool {
        let handle = self.ws_handle.lock().await;
        handle.is_some()
    }

    /// Close the WebSocket connection
    pub async fn close(mut self) -> CoinbaseResult<()> {
        // Close the command channel
        {
            let mut handle = self.ws_handle.lock().await;
            *handle = None;
        }

        // Wait for background task to finish
        if let Some(task_handle) = self.task_handle.take() {
            match task_handle.await {
                Ok(result) => result,
                Err(e) => {
                    log::error!("WebSocket task join error: {:?}", e);
                    Err(CoinbaseError::other("Failed to join WebSocket task"))
                }
            }
        } else {
            Ok(())
        }
    }
}

impl Drop for WebsocketHandle {
    fn drop(&mut self) {
        if let Some(task_handle) = self.task_handle.take() {
            // Try to abort the task gracefully
            task_handle.abort();
        }
    }
}

// Implement compatibility methods for existing API
impl WebsocketStream {
    /// Legacy compatibility: get message receiver
    pub fn into_receiver(self) -> mpsc::UnboundedReceiver<ClientMessage> {
        self.message_rx
    }
}
