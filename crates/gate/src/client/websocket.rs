use std::io;
use std::time::Duration;
use std::time::Instant;

use actix::io::SinkWrite;
use actix::prelude::*;
use actix_codec::Framed;
use actix_http::ws::Codec;
use actix_web_actors::ws;
use awc::BoxedSocket;
use futures::channel::mpsc;
use futures::stream::SplitSink;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

use crate::client::RestClient;
use crate::error::GateError;
use crate::error::GateResult;
use crate::websocket::order_book::OrderBookRequest;
use crate::websocket::request::WsRequest;
use crate::websocket::request::WsRequestEvent;
use crate::websocket::response::Event;
use crate::websocket::response::WsResponse;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(actix::Message, Clone, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
struct M<T>(pub T);

pub struct WebsocketStream {
    tx: WebsocketStreamTx,
    rx: mpsc::UnboundedReceiver<WsResponse>,
}

pub struct WebsocketStreamTx {
    addr: Addr<Websocket>,
}

pub struct Websocket {
    sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>,
    tx: mpsc::UnboundedSender<WsResponse>,
    latest_heartbeat_time: Instant,
}

impl Actor for Websocket {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat_task(ctx);
    }
}

/// Handler for `ws::Message`.
impl StreamHandler<Result<ws::Frame, ws::ProtocolError>> for Websocket {
    fn handle(&mut self, msg: Result<ws::Frame, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                log::warn!("WebSocket broken: {e:?}");
                return ctx.stop();
            }
        };

        match msg {
            ws::Frame::Ping(msg) => {
                self.latest_heartbeat_time = Instant::now();
                if let Err(_msg) = self.sink.write(ws::Message::Pong(msg)) {
                    log::warn!("Failed to send Pong. Disconnecting.");
                    ctx.stop()
                }
            }
            ws::Frame::Pong(_) => {
                self.latest_heartbeat_time = Instant::now();
            }
            ws::Frame::Binary(_bin) => {
                log::warn!("unexpected binary message (ignored)");
            }
            ws::Frame::Text(msg) => match serde_json::from_slice(&msg) {
                Err(e) => {
                    log::error!(
                        "Failed to deserialize server message: {e:?}. Message: {}",
                        String::from_utf8_lossy(&msg)
                    )
                }
                Ok(WsResponse {
                    event: Event::Pong(Ok(())),
                    ..
                }) => {
                    self.latest_heartbeat_time = Instant::now();
                }
                Ok(msg) => {
                    if let Err(e) = self.tx.unbounded_send(msg) {
                        log::warn!("Failed to notify downstream: {e:?}");
                        ctx.stop()
                    }
                }
            },
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

impl Handler<M<WsRequest>> for Websocket {
    type Result = ();

    fn handle(&mut self, M(msg): M<WsRequest>, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&msg).expect("json encode");
        log::debug!("Sending to server: `{msg}`");
        if let Err(_msg) = self.sink.write(ws::Message::Text(msg.into())) {
            ctx.stop();
        }
    }
}

impl Websocket {
    pub(crate) fn new(
        sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>,
        tx: mpsc::UnboundedSender<WsResponse>,
    ) -> Self {
        Self {
            sink,
            tx,
            latest_heartbeat_time: Instant::now(),
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn start_heartbeat_task(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.latest_heartbeat_time) > CLIENT_TIMEOUT {
                log::warn!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            if let Err(_msg) = act.sink.write(ws::Message::Ping("".into())) {
                log::warn!("Websocket client failed to send ping, stopping!");
                ctx.stop()
            };
        });
    }
}

impl WebsocketStream {
    pub async fn connect<S>(api_client: RestClient<S>, url: Url) -> GateResult<Self> {
        use futures::StreamExt;
        log::debug!("Connecting WS: {}", url.as_str());

        let (response, connection) = api_client.client().ws(url.as_str()).connect().await?;
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
        self.addr
            .send(M(request))
            .await
            .map_err(|_e| GateError::IoError(io::ErrorKind::ConnectionAborted.into()))
    }

    /// Subscribe or unsubscribe from order book snapshots
    pub async fn order_book(
        &self,
        event: WsRequestEvent,
        payload: OrderBookRequest,
    ) -> GateResult<()> {
        self.addr
            .send(M(WsRequest::order_book(event, payload)))
            .await
            .map_err(|_e| GateError::IoError(io::ErrorKind::ConnectionAborted.into()))
    }
}
