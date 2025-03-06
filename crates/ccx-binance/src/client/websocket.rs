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
use crate::types::ws_requests::WsMessageId;
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
    Subscribe(Cow<'a, [StreamName]>),
    Unsubscribe(Cow<'a, [StreamName]>),
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
}

async fn upstream_loop(
    mut sender: soketto::Sender<impl futures::AsyncRead + futures::AsyncWrite + Unpin>,
    mut receiver: soketto::Receiver<impl futures::AsyncRead + futures::AsyncWrite + Unpin>,
    mut cmd_rx: fmpsc::Receiver<Command>,
    mut stream_tx: fmpsc::Sender<Vec<u8>>,
) {
    let mut next_id = 1;
    let mut requests: HashMap<i64, oneshot::Sender<Vec<u8>>> = HashMap::new();
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
                    id: Some(WsMessageId::Int(id)),
                    payload: match &cmd {
                        WsCommand::SubscribeOne(list) => StreamCommand::Subscribe((&*list).into()),
                        WsCommand::SubscribeList(list) => StreamCommand::Subscribe((&*list).into()),
                        WsCommand::UnsubscribeOne(list) => StreamCommand::Unsubscribe((&*list).into()),
                        WsCommand::UnsubscribeList(list) => StreamCommand::Unsubscribe((&*list).into()),
                    }
                }).unwrap();
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
                            let meta = preparse_meta(&buf).unwrap();
                            if let Some(WsMessageId::Int(id)) = meta.id {
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
                        tracing::debug!("error receiving message: {:?}", reason);
                        break;
                    }
                }
            }
        }
    }
}

fn preparse_meta(data: &[u8]) -> Result<WsMeta<IgnorePayload>, serde_json::Error> {
    Ok(serde_json::from_slice(data)?)
}
