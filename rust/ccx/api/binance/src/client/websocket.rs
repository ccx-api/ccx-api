use std::io;

use actix_web::rt::Arbiter;
use awc::ws;
use futures::channel::mpsc;
use futures::SinkExt;
use string_cache::DefaultAtom as Atom;
use url::Url;

use super::*;
use crate::error::*;
use crate::{WsCommand, WsEvent, WsStream, WsSubscription};

pub struct WebsocketClient {
    tx: WebsocketClientTx,
    rx: mpsc::Receiver<WsEvent>,
}

pub struct WebsocketClientTx {
    tx: mpsc::Sender<WsCommand>,
}

impl WebsocketClient {
    pub async fn connect(api_client: RestClient, url: Url) -> LibResult<Self> {
        use futures::stream::StreamExt;

        log::debug!("Connecting WS: {}", url.as_str());

        let (_resp, connection) = api_client.client().ws(url.as_str()).connect().await?;

        // Сообщения на / с апстрима, приходящие через сокет.
        let (mut up_sink, mut up_stream) = connection.split();
        // Сообщения на / с апстрима, для внутреннего использования.
        let (mut up_tx, up_rx) = futures::channel::mpsc::channel(0);
        // Внутренний передатчик сообщений клиенту и клиентский получатель сообщений.
        let (mut dn_sink, dn_rx) = futures::channel::mpsc::channel(0);
        // Клиентский передатчик сообщений и внутренний слушатель.
        let (dn_tx, mut dn_stream) = futures::channel::mpsc::channel(0);

        Arbiter::spawn(async move {
            // Перенаправляем внутренние сообщения апстриму в канал апстрима.
            let _ = up_sink.send_all(&mut up_rx.map(Ok)).await;
        });

        Arbiter::spawn({
            let mut up_tx = up_tx.clone();
            async move {
                // Получаем команды от клиента и передаём их по внутреннему каналу апстриму.
                while let Some(cmd) = dn_stream.next().await {
                    match serde_json::to_string(&cmd) {
                        Ok(msg) => {
                            let _ = up_tx.send(ws::Message::Text(msg)).await;
                        }
                        Err(e) => {
                            log::warn!("Communication error: {:?}", e)
                        }
                    }
                }
                // Клиентский канал закрылся, закрываем соединение с апстримом.
                let _ = up_tx.send(ws::Message::Close(None)).await;
            }
        });

        Arbiter::spawn(async move {
            let res: LibResult<()> = async {
                // Слушаем сообщения апстрима, по внутреннему каналу отвечаем апстриму на пинги,
                // передаём декодированные события клиенту.
                'iter_frames: while let Some(frame) = up_stream.next().await {
                    let res: LibResult<Option<WsEvent>> = async {
                        match frame? {
                            ws::Frame::Close(e) => {
                                up_tx.send(ws::Message::Close(e)).await.map_err(|_e| {
                                    LibError::IoError(io::ErrorKind::ConnectionAborted.into())
                                })?;
                                Ok(None)
                            }
                            ws::Frame::Ping(d) => {
                                up_tx.send(ws::Message::Pong(d)).await.map_err(|_e| {
                                    LibError::IoError(io::ErrorKind::ConnectionAborted.into())
                                })?;
                                Ok(None)
                            }
                            ws::Frame::Pong(_) => Ok(None),
                            ws::Frame::Text(msg) => Ok(Some(serde_json::from_slice(&msg)?)),
                            ws::Frame::Binary(_d) => Ok(None),
                            ws::Frame::Continuation(_d) => Ok(None),
                        }
                    }
                    .await;
                    match res.transpose() {
                        Some(Ok(res)) => {
                            if let Err(_e) = dn_sink.send(res).await {
                                let _ = up_tx.send(ws::Message::Close(None)).await;
                                break 'iter_frames;
                            }
                        }
                        Some(Err(e)) => {
                            log::warn!("Communication error 1: {:?}", e);
                            if let LibError::IoError(e) = &e {
                                if e.kind() != io::ErrorKind::ConnectionAborted {
                                    let _ = up_tx.send(ws::Message::Close(None)).await;
                                }
                            }
                            break 'iter_frames;
                        }
                        None => {}
                    }
                }
                Ok(())
            }
            .await;
            if let Err(e) = res {
                eprintln!("Communication error 2: {:?}", e)
                // let _ = dn_sink.send(Err(e)).await;
            }
        });

        Ok(WebsocketClient {
            tx: WebsocketClientTx { tx: dn_tx },
            rx: dn_rx,
        })

        // ClientBuilder::from_url(&self.url)
        //     .async_connect(None)
        //     .map(move |(duplex, _)| {
        //         //                 let (sink, stream) = duplex.split();
        //         //                 let (tx, rx) = futures01::sync::mpsc::channel(0);
        //         //                 let rx = sink
        //         //                     .sink_map_err(drop)
        //         //                     .send_all(rx.filter_map(|m| m).map_err(drop))
        //         //                     .map(drop);
        //         //                 tokio::runtime::current_thread::spawn(rx);
        //         //                 let stream = stream
        //         //                     .and_then(move |message| {
        //         //                         // dbg!("Received Message: {:?}", message);
        //         //                         let (up, dn) = match message {
        //         //                             OwnedMessage::Close(e) => (Some(OwnedMessage::Close(e)), None),
        //         //                             OwnedMessage::Ping(d) => (Some(OwnedMessage::Pong(d)), None),
        //         //                             OwnedMessage::Pong(_) => (None, None),
        //         //                             OwnedMessage::Text(msg) => (None, Some(msg)),
        //         //                             OwnedMessage::Binary(_d) => {
        //         //                                 // warn!("Unexpected binary data {:?}", d);
        //         //                                 (None, None)
        //         //                             }
        //         //                         };
        //         //                         tx.clone()
        //         //                             .send(up)
        //         //                             .map_err(|_| {
        //         //                                 WebSocketError::IoError(io::ErrorKind::ConnectionAborted.into())
        //         //                             })
        //         //                             .map(|_| dn)
        //         //                     })
        //         //                     .filter_map(|v: Option<String>| v)
        //         //                     .map_err(Error::from)
        //         //                     .and_then(|msg| serde_json::from_str(&msg).map_err(Error::from))
        //         //                     .compat();
        //         //                 stream
        //     })
        //     .map_err(Error::from)
        //     .await
    }

    pub fn split(self) -> (WebsocketClientTx, mpsc::Receiver<WsEvent>) {
        (self.tx, self.rx)
    }
}

impl WebsocketClientTx {
    pub async fn subscribe(
        &mut self,
        market: impl Into<Atom>,
        stream: WsStream,
    ) -> std::result::Result<(), mpsc::SendError> {
        let cmd = WsCommand::Subscribe1([WsSubscription::new(market, stream)]);
        Ok(self.tx.send(cmd).await?)
    }
}
