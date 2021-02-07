use std::io;
use std::time::{Duration, Instant};

use actix::io::SinkWrite;
use actix::prelude::*;
use actix_codec::Framed;
use actix_http::ws::Codec;
use actix_web_actors::ws;
use awc::BoxedSocket;
use futures::channel::mpsc;
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;
use url::Url;

use crate::client::RestClient;
use crate::error::LibResult;
use crate::util::Seq;
use crate::{LibError, UpstreamApiRequest, WsCommand, WsEvent, WsStream, WsSubscription, UpstreamWebsocketMessage};

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(actix::Message, Clone, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
struct M<T>(pub T);

pub struct WebsocketStream {
    tx: WebsocketStreamTx,
    rx: mpsc::UnboundedReceiver<UpstreamWebsocketMessage<WsEvent>>,
}

pub struct WebsocketStreamTx {
    addr: Addr<Websocket>,
}

pub struct Websocket {
    sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>,
    tx: mpsc::UnboundedSender<UpstreamWebsocketMessage<WsEvent>>,
    hb: Instant,
    id_seq: Seq<u64>,
}

impl Actor for Websocket {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`.
impl StreamHandler<Result<ws::Frame, ws::ProtocolError>> for Websocket {
    fn handle(&mut self, msg: Result<ws::Frame, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                log::warn!("WebSocket broken: {:?}", e);
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Frame::Ping(msg) => {
                self.hb = Instant::now();
                if self.sink.write(ws::Message::Pong(msg)).is_some() {
                    log::warn!("Failed to send Pong. Disconnecting.");
                    ctx.stop()
                }
            }
            ws::Frame::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Frame::Binary(_bin) => {
                log::warn!("unexpected binary message (ignored)");
            }
            ws::Frame::Text(msg) => {
                log::debug!("json message from server: {:?}", msg);
                match serde_json::from_slice(&msg) {
                    Err(e) => {
                        log::warn!("Failed to deserialize server message: {:?}", e);
                    }
                    Ok(msg) => {
                        if let Err(e) = self.tx.unbounded_send(msg) {
                            log::warn!("Failed to notify downstream: {:?}", e);
                            ctx.stop()
                        }
                    }
                }
            }
            ws::Frame::Close(_) => {
                ctx.stop();
            }
            ws::Frame::Continuation(_) => {
                ctx.stop();
            }
        }
    }
}

impl actix::io::WriteHandler<ws::ProtocolError> for Websocket {}

impl Handler<M<WsCommand>> for Websocket {
    type Result = ();

    fn handle(&mut self, M(cmd): M<WsCommand>, ctx: &mut Self::Context) {
        let msg = UpstreamApiRequest {
            id: self.id_seq.next(),
            payload: cmd,
        };
        let msg = serde_json::to_string(&msg).expect("json encode");
        log::debug!("Sending to server: `{}`", msg);
        if self.sink.write(ws::Message::Text(msg)).is_some() {
            ctx.stop();
        }
    }
}

impl Websocket {
    #[rustfmt::skip]
    pub(crate) fn new(
        sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>,
        tx: mpsc::UnboundedSender<UpstreamWebsocketMessage<WsEvent>>,
    ) -> Self {
        let hb = Instant::now();
        let id_seq = Seq::new();
        Self { sink, tx, hb, id_seq }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::warn!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            if act.sink.write(ws::Message::Ping("".into())).is_some() {
                log::warn!("Websocket client failed to send ping, stopping!");
                ctx.stop()
            };
        });
    }
}

impl WebsocketStream {
    pub async fn connect(api_client: RestClient, url: Url) -> LibResult<Self> {
        use futures::StreamExt;

        log::debug!("Connecting WS: {}", url.as_str());

        let (response, connection) = api_client.client_h1().ws(url.as_str()).connect().await?;
        log::debug!("{:?}", response);

        let (sink, stream) = connection.split();
        let (tx, rx) = mpsc::unbounded();
        let addr = Websocket::create(move |ctx| {
            Websocket::add_stream(stream, ctx);
            Websocket::new(SinkWrite::new(sink, ctx), tx)
        });

        let tx = WebsocketStreamTx { addr };
        Ok(WebsocketStream { tx, rx })
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
    pub async fn subscribe(&self, market: impl Into<Atom>, stream: WsStream) -> LibResult<()> {
        let cmd = WsCommand::Subscribe1([WsSubscription::new(market, stream)]);
        Ok(self
            .addr
            .send(M(cmd))
            .await
            .map_err(|_e| LibError::IoError(io::ErrorKind::ConnectionAborted.into()))?)
    }
}
