use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;

use ccx_lib::websocket::{WebSocketConnectError, websocket_builder};
use futures::SinkExt;
use futures::StreamExt;
use futures::channel::mpsc as fmpsc;
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};
use soketto::Data;
use soketto::Incoming;
use url::Url;

use crate::types::ws_requests::IgnorePayload;
use crate::types::ws_requests::WsMeta;
use crate::types::ws_stream_name::StreamName;

pub struct WebSocketClient {
    cmd_tx: fmpsc::Sender<Command>,
}

struct Command {
    cmd: WsCommand,
    tx: oneshot::Sender<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub enum WsCommand {
    SubscribeOne([StreamName; 1]),
    SubscribeList(Vec<StreamName>),
    UnsubscribeOne([StreamName; 1]),
    UnsubscribeList(Vec<StreamName>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "method",
    content = "params",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum StreamCommand<'a> {
    Subscription(Cow<'a, [StreamName]>),
    Unsubscription(Cow<'a, [StreamName]>),
}

impl WebSocketClient {
    pub async fn connect(
        stream_url: Url,
    ) -> Result<(Self, fmpsc::Receiver<Vec<u8>>), WebSocketConnectError> {
        let builder = websocket_builder(&stream_url).await?;
        let (sender, receiver) = builder.finish();
        let (cmd_tx, cmd_rx) = fmpsc::channel(4);
        let (stream_tx, stream_rx) = fmpsc::channel(4);
        tokio::spawn(async move {
            upstream_loop(sender, receiver, cmd_rx, stream_tx).await;
        });

        Ok((Self { cmd_tx }, stream_rx))
    }

    pub async fn subscribe(&mut self, stream_name: StreamName) {
        tracing::debug!(?stream_name, "Subscribing to stream");

        let (tx, rx) = oneshot::channel();

        let _ = self
            .cmd_tx
            .send(Command {
                cmd: WsCommand::SubscribeOne([stream_name]),
                tx,
            })
            .await
            .inspect_err(|error| {
                tracing::error!(?error, "Failed to send request due to error");
            });

        let _ = rx.await.inspect_err(|error| {
            tracing::error!(?error, "Error while awaiting the response");
        });
    }
}

async fn upstream_loop(
    mut sender: soketto::Sender<impl futures::AsyncRead + futures::AsyncWrite + Unpin>,
    mut receiver: soketto::Receiver<impl futures::AsyncRead + futures::AsyncWrite + Unpin>,
    mut cmd_rx: fmpsc::Receiver<Command>,
    mut stream_tx: fmpsc::Sender<Vec<u8>>,
) {
    let mut next_id = 1u64;
    let mut requests = HashMap::new();
    let mut buf = Vec::new();
    loop {
        tokio::select! {
            cmd = cmd_rx.next() => {
                let Command { cmd, tx } = match cmd {
                    Some(cmd) => cmd,
                    None => {
                        tracing::debug!("command channel closed");
                        break;
                    }
                };
                let id = next_id;
                next_id += 1;
                requests.insert(id, tx);
                tracing::debug!("command: {id} {cmd:?}");
                let message = serde_json::to_string(&WsMeta {
                    id: Some(id),
                    payload: match &cmd {
                        WsCommand::SubscribeOne(list) => StreamCommand::Subscription((&*list).into()),
                        WsCommand::SubscribeList(list) => StreamCommand::Subscription((&*list).into()),
                        WsCommand::UnsubscribeOne(list) => StreamCommand::Unsubscription((&*list).into()),
                        WsCommand::UnsubscribeList(list) => StreamCommand::Unsubscription((&*list).into()),
                    }
                }).unwrap();
                dbg!(&message);
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
                            debug_assert_eq!(count, buf.len());
                            dbg!(String::from_utf8_lossy(&buf));

                            let meta = preparse_meta(&buf).unwrap();
                            if let Some(id) = meta.id {
                                if let Some(tx) = requests.remove(&id) {
                                    let _ = tx.send(mem::take(&mut buf));
                                }
                            } else {
                                let _ = stream_tx.send(mem::take(&mut buf)).await;
                            }
                        }
                        Incoming::Pong(_data) => {
                                tracing::debug!("received pong");
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
        }
    }
}

fn preparse_meta(data: &[u8]) -> Result<WsMeta<IgnorePayload>, serde_json::Error> {
    Ok(serde_json::from_slice(data)?)
}
