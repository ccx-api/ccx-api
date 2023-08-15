use std::collections::HashMap;
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
use url::Url;

use crate::client::RestClient;
use crate::error::{BitstampError, BitstampResult};
/*
use crate::ws_stream::UpstreamApiRequest;
use crate::ws_stream::UpstreamWebsocketMessage;
*/
use crate::ws_stream::{Event, SystemEvent, WsCommand, WsEvent, WsSubscription};

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
/// Interval between automatic reconnections.
///
/// According to documentation every connection older than 90 days will be
/// automatically dropped.
const RECONNECT_INTERVAL: Duration = Duration::from_secs(30 * 24 * 60 * 60);

#[derive(actix::Message, Clone, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
struct M<T>(pub T);

#[derive(actix::Message, Clone, Debug)]
#[rtype(result = "()")]
struct ReconnectSocket;

pub struct WebsocketStream {
    tx: WebsocketStreamTx,
    rx: mpsc::UnboundedReceiver<WsEvent>,
}

pub struct WebsocketStreamTx {
    addr: Addr<Websocket>,
}

pub struct Websocket {
    api_client: awc::Client,
    ws_url: Url,
    tx: mpsc::UnboundedSender<WsEvent>,

    channels: HashMap<WsSubscription, bool>,

    inner: Option<InnerSocket>,
}

struct InnerSocket {
    sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>,
    hb: Instant,
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
                self.inner_mut().hb = Instant::now();
                if let Err(_msg) = self.inner_mut().sink.write(ws::Message::Pong(msg)) {
                    log::warn!("Failed to send Pong. Disconnecting.");
                    ctx.stop()
                }
            }
            ws::Frame::Pong(_) => {
                self.inner_mut().hb = Instant::now();
            }
            ws::Frame::Binary(_bin) => {
                log::warn!("unexpected binary message (ignored)");
            }
            ws::Frame::Text(msg) => {
                let res = serde_json::from_slice(&msg);
                if res.is_err() {
                    log::error!(
                        "json message from server: {}",
                        String::from_utf8_lossy(&msg)
                    );
                }

                let event = match res {
                    Err(e) => {
                        log::error!("Failed to deserialize server message: {:?}", e);
                        return;
                    }
                    Ok(msg) => msg,
                };

                match event {
                    Event::Client(ev) => {
                        if let Err(e) = self.tx.unbounded_send(ev) {
                            log::warn!("Failed to notify downstream: {:?}", e);
                            ctx.stop()
                        }
                    }
                    Event::System(ev) => match ev {
                        SystemEvent::ReconnectRequest => {
                            log::debug!("Reconnect request received");
                            ctx.notify(ReconnectSocket);
                        }
                        SystemEvent::SubscriptionSucceeded { channel } => {
                            let subscription = channel.into();
                            if !self.channels.contains_key(&subscription) {
                                log::warn!(
                                    "Successfully subscribed to {:?}. But it was \
                                         not found in list of active subscriptions",
                                    subscription,
                                );
                            }
                            self.channels.insert(subscription, true);
                        }
                        SystemEvent::Error { channel, data } => {
                            log::error!(
                                "Websocket Channel({}) returned error: {:?}",
                                channel,
                                data
                            );
                        }
                        SystemEvent::Heartbeat => {}
                    },
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
        let msg = serde_json::to_string(&cmd).expect("json encode");
        log::debug!("Sending to server: `{}`", msg);
        if let Err(_) = self.inner_mut().sink.write(ws::Message::Text(msg.into())) {
            ctx.stop();
        }

        match cmd {
            WsCommand::Subscribe(cmd) => {
                self.channels.entry(cmd).or_default();
            }
            WsCommand::Unsubscribe(cmd) => {
                self.channels.remove(&cmd);
            }
        };
    }
}

impl Handler<ReconnectSocket> for Websocket {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _: ReconnectSocket, _: &mut Self::Context) -> Self::Result {
        use futures::StreamExt as _;

        let api_client = self.api_client.clone();
        let ws_url = self.ws_url.clone();

        let fut = async move { api_client.ws(ws_url.as_str()).connect().await };
        let fut = fut.into_actor(self).then(|res, act, ctx| {
            let (resp, connection) = match res {
                Ok((resp, connection)) => (resp, connection),
                Err(err) => {
                    log::error!("Socket connection was not initialized: {}", err);
                    ctx.stop();
                    return fut::ready(());
                }
            };
            log::debug!("Websocket response: {:?}", resp);
            let (sink, stream) = connection.split();

            ctx.add_stream(stream);
            act.inner = Some(InnerSocket {
                sink: SinkWrite::new(sink, ctx),
                hb: Instant::now(),
            });

            // Resubscribe to previous subscriptions.
            let old_subscriptions = std::mem::take(&mut act.channels);
            for (subscription, _) in old_subscriptions {
                ctx.notify(M(WsCommand::Subscribe(subscription)));
            }

            fut::ready(())
        });
        Box::pin(fut)
    }
}

impl Websocket {
    pub fn new(client: awc::Client, url: Url, tx: mpsc::UnboundedSender<WsEvent>) -> Self {
        Self {
            api_client: client,
            ws_url: url,
            tx,
            channels: HashMap::new(),
            inner: None,
        }
    }

    fn inner_mut(&mut self) -> &mut InnerSocket {
        self.inner.as_mut().expect("Uninitialized")
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.inner_mut().hb) > CLIENT_TIMEOUT {
                log::warn!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            if let Err(_msg) = act.inner_mut().sink.write(ws::Message::Ping("".into())) {
                log::warn!("Websocket client failed to send ping, stopping!");
                ctx.stop()
            };
        });

        ctx.run_interval(RECONNECT_INTERVAL, move |_, ctx| {
            ctx.notify(ReconnectSocket);
        });
    }
}

impl WebsocketStream {
    pub async fn connect<S: crate::client::BitstampSigner>(
        api_client: RestClient<S>,
        url: Url,
    ) -> BitstampResult<Self> {
        log::debug!("Connecting WS: {}", url.as_str());

        let client = api_client.client();

        let (tx, rx) = mpsc::unbounded();

        // Initialize new socket and reconnect.
        let addr = Websocket::new(client, url, tx).start();
        addr.send(ReconnectSocket)
            .await
            .map_err(|_| BitstampError::IoError(io::ErrorKind::ConnectionAborted.into()))?;

        let tx = WebsocketStreamTx { addr };
        Ok(WebsocketStream { tx, rx })
    }

    pub fn split(self) -> (WebsocketStreamTx, mpsc::UnboundedReceiver<WsEvent>) {
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
    pub async fn subscribe_one(
        &self,
        subscription: impl Into<WsSubscription>,
    ) -> BitstampResult<()> {
        let cmd = WsCommand::Subscribe(subscription.into());
        self.addr
            .send(M(cmd))
            .await
            .map_err(|_e| BitstampError::IoError(io::ErrorKind::ConnectionAborted.into()))
    }
}
