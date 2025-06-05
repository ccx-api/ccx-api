use ccx_lib::websocket::{WebSocketConnectError, websocket_builder};
use futures::channel::mpsc::{self, SendError};
use futures::{SinkExt, Stream, StreamExt};
use soketto::{Data, Incoming};
use tracing::Instrument;
use url::Url;

use crate::api_ws::WsRequest;
use crate::api_ws::WsResponse;

pub struct WebSocketClient {
    ws_receiver: mpsc::Receiver<WsResponse>,
    ws_sender: mpsc::Sender<WsRequest>,
}

#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum WebSocketError {
    Channel(SendError),
}

impl WebSocketClient {
    pub async fn connect(stream_url: &Url) -> Result<Self, WebSocketConnectError> {
        let builder = websocket_builder().stream_url(stream_url).call().await?;
        let (ws_sender, mut rx_request) = mpsc::channel(4);
        let (mut tx_response, ws_receiver) = mpsc::channel(4);
        tokio::spawn(async move {
            let (mut sender, mut receiver) = builder.finish();
            let mut buf = Vec::new();

            loop {
                buf.clear();

                let _ = tokio::select! {
                    request = rx_request.next() => {
                        let Some(request) = request else {
                            tracing::debug!("command channel closed");
                            break;
                        };

                        let message = match serde_json::to_string(&request) {
                            Ok(msg) => msg,
                            Err(error) => {
                                tracing::error!(?error, "message conversion error");
                                continue;
                            }
                        };

                        sender.send_text(&message).await.unwrap();
                    }
                    message = receiver.receive(&mut buf) => {
                        match message {
                            Ok(incoming) => match incoming {
                                Incoming::Data(data) => {
                                    let count = match data {
                                        Data::Text(count) | Data::Binary(count) => count,
                                    };
                                    tracing::debug!("received {count} bytes");

                                    let response: WsResponse = match serde_json::from_slice(&buf) {
                                        Ok(msg) => msg,
                                        Err(error) => {
                                            tracing::error!(?error, "message conversion error");
                                            continue;
                                        }
                                    };

                                    let _ = tx_response.send(response).await;
                                }
                                Incoming::Pong(_data) => {
                                    tracing::trace!("received pong");
                                }
                                Incoming::Closed(close_reason) => {
                                    tracing::debug!("connection closed: {:?}", close_reason);
                                    break;
                                }
                            },
                            Err(reason) => {
                                if matches!(&reason, soketto::connection::Error::UnexpectedOpCode(_)) {
                                    tracing::warn!("error receiving message: {:?}", reason);

                                    continue;
                                } else {
                                    tracing::warn!("error receiving message: {:?}", reason);
                                    break;
                                }
                            }
                        }
                    }
                };
            }
        }
        .instrument(tracing::debug_span!("websocket_handler", %stream_url)));

        Ok(Self {
            ws_receiver,
            ws_sender,
        })
    }

    pub async fn send(&mut self, request: WsRequest) -> Result<(), WebSocketError> {
        Ok(self.ws_sender.send(request).await?)
    }
}

impl Stream for WebSocketClient {
    type Item = WsResponse;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this: &mut WebSocketClient = self.get_mut();
        let stream = &mut this.ws_receiver;
        stream.poll_next_unpin(cx)
    }
}
