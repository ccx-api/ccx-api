use std::result::Result as StdResult;
use std::time::Duration;
use std::time::Instant;

use actix::io::SinkWrite;
use actix::io::WriteHandler;
use actix::Actor;
use actix::ActorContext;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::StreamHandler;
use actix_codec::Framed;
use awc::error::WsProtocolError;
use awc::ws::Codec;
use awc::ws::Frame;
use awc::ws::Message;
use awc::BoxedSocket;
use bytes::Bytes;
// use futures::channel::mpsc::UnboundedReceiver;
// use futures::channel::mpsc::UnboundedSender;
use futures::channel::mpsc::Receiver;
use futures::channel::mpsc::Sender;
use futures::stream::SplitSink;
use futures::StreamExt;

use super::FinerySigner;
use super::RestClient;
use crate::error::LibError;
use crate::error::LibResult;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::WsResponse;

mod implement;

const RECONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const SERVER_TIMEOUT: Duration = Duration::from_secs(60);

const HEADER_KEY: &str = "EFX-Key";
const HEADER_SIGN: &str = "EFX-Sign";
const HEADER_CONTENT: &str = "EFX-Content";

pub type WsSender = Sender<LibResult<WsResponse>>;
pub type WsReceiver = Receiver<LibResult<WsResponse>>;

#[derive(Clone)]
pub struct WebSocket {
    addr: Addr<WebSocketActor>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EfxContent {
    nonce: u64,
    timestamp: u64,
}

impl WebSocket {
    pub(crate) async fn close(self) {
        self.die().await;
    }

    pub async fn connect<S: FinerySigner>(
        tx: WsSender,
        api_client: RestClient<S>,
        nonce: Nonce,
        time: Time,
    ) -> LibResult<WebSocket> {
        let framed = WebSocket::raw_connect(api_client, nonce, time).await?;
        let (sink, stream) = framed.split();

        let addr = WebSocketActor::create(move |ctx| {
            WebSocketActor::add_stream(stream, ctx);
            WebSocketActor::new(tx, SinkWrite::new(sink, ctx))
        });

        Ok(Self { addr })
    }

    async fn raw_connect<S: FinerySigner>(
        api_client: RestClient<S>,
        nonce: Nonce,
        time: Time,
    ) -> LibResult<Framed<BoxedSocket, Codec>> {
        let url = api_client.stream_url();
        log::debug!("raw_connect :: {}", url.as_str());
        let client = api_client.client();
        let request = client.ws(url.as_str());
        let content = EfxContent {
            nonce: nonce.0,
            timestamp: time.0,
        };
        let content = serde_json::to_string(&content)?;
        let signature = api_client.signer().sign_data(&content).await?;

        let request = request.header(HEADER_KEY, api_client.key());
        let request = request.header(HEADER_SIGN, signature);
        let request = request.header(HEADER_CONTENT, content);
        let (response, framed) = request.connect().await.map_err(|e| {
            LibError::other(format!("Failed to open sign storage websocket: {:?}", e))
        })?;

        log::debug!("response: {:?}", response);

        Ok(framed)
    }
}

struct WebSocketActor {
    tx: WsSender,
    hb: Instant,
    sink: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
}

impl WebSocketActor {
    fn new(
        tx: WsSender,
        sink: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
    ) -> WebSocketActor {
        let hb = Instant::now();
        Self { tx, hb, sink }
    }

    pub fn send(&mut self, msg: String, ctx: &mut Context<Self>) -> LibResult<()> {
        log::debug!("send msg :: {}", msg);

        if let Err(_msg) = self.sink.write(Message::Text(msg.into())) {
            self.stop_context(ctx);
            return Err(LibError::other(
                "WebSocketActor failed to send message to server ws.",
            ));
        }
        Ok(())
    }

    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(HEARTBEAT_INTERVAL, move |act, ctx| {
            if act.hb.elapsed() > SERVER_TIMEOUT {
                act.stop_context(ctx);
            } else {
                let ping_message = Bytes::from_static(b"");
                match act.sink.write(Message::Ping(ping_message)) {
                    Ok(()) => act.hb(ctx),
                    Err(_msg) => {
                        log::error!("WebSocketActor failed to ping server ws");
                        act.stop_context(ctx);
                    }
                };
            }
        });
    }

    fn notify(&mut self, bytes: Bytes, ctx: &mut Context<Self>) {
        let mut tx = self.tx.clone();
        let response = WsResponse::try_from(bytes);
        if let Err(error) = tx.start_send(response) {
            log::error!("Failed notify by ws finary: {:?}", error);
            self.stop_context(ctx);
        }
        // actix_rt::spawn(async move {
        //     let response = WsResponse::try_from(bytes);
        //     let _ = tx.send(response).await;
        // });
    }
}

impl Actor for WebSocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        log::debug!("WebSocketActor started");
        self.hb(ctx)
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        log::debug!("WebSocketActor stopped");
    }
}

impl StreamHandler<StdResult<Frame, WsProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: StdResult<Frame, WsProtocolError>, ctx: &mut Context<Self>) {
        if let Ok(frame) = msg {
            self.hb = Instant::now();
            log::debug!("StreamHandler frame :: {:?}", frame);
            match frame {
                Frame::Text(bytes) | Frame::Binary(bytes) => {
                    self.notify(bytes, ctx);
                }
                Frame::Ping(v) => {
                    if let Err(_msg) = self.sink.write(Message::Pong(v)) {
                        log::error!("WebSocketActor failed to respond to server ws");
                        self.stop_context(ctx);
                    };
                }
                Frame::Pong(_) => {
                    self.hb = Instant::now();
                }
                Frame::Continuation(_) => {}
                Frame::Close(reason) => {
                    log::error!("WebSocketActor close connection with reason: {:?}", reason);
                    self.stop_context(ctx);
                }
            }
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        log::debug!("WebSocketActor StreamHandler started");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        log::debug!("WebSocketActor StreamHandler finished");
        self.stop_context(ctx);
    }
}

// impl Supervised for WebSocketActor
// where
//     S: FinerySigner,
// {
//     fn restarting(&mut self, ctx: &mut Context<Self>) {
//         log::debug!("WebSocketActor restarting :: {:?}", ctx);
//         let old_sink = self.sink.take();
//         if let Some(mut sink) = old_sink {
//             log::debug!("WebSocketActor restarting sink.close");
//             sink.close();
//         }

//         let api_client = self.api_client.clone();
//         let fut = async move {
//             delay_for(RECONNECT_TIMEOUT).await;
//             WebSocket::::raw_connect(api_client, nonce: Nonce, time: Time).await
//         };
//         fut.into_actor(self)
//             .then(|res, act, ctx| {
//                 match res {
//                     Ok(framed) => {
//                         log::debug!("WebSocketActor restarting framed");
//                         let (sink, stream) = framed.split();
//                         Self::add_stream(stream, ctx);
//                         act.sink = Some(SinkWrite::new(sink, ctx));
//                         act.hb = Instant::now();
//                     }
//                     Err(error) => {
//                         log::error!("Failed re/connect to sign storage: {:?}", error);
//                         act.stop_context(ctx);
//                     }
//                 }
//                 fut::ready(())
//             })
//             .wait(ctx);
//     }
// }

impl WebSocketActor {
    pub fn stop_context(&mut self, ctx: &mut Context<Self>) {
        self.tx.close_channel();
        ctx.stop();
    }
}

impl WriteHandler<WsProtocolError> for WebSocketActor {
    fn finished(&mut self, ctx: &mut Self::Context) {
        log::debug!("WebSocketActor WriteHandler finished");
        self.stop_context(ctx);
    }
}
