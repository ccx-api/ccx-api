use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

use actix::io::SinkWrite;
use actix::prelude::*;
use actix_codec::Framed;
use actix_http::ws::Codec;
use actix_http::ws::Item as FrameItem;
use actix_web_actors::ws;
use awc::BoxedSocket;
use futures::channel::mpsc;
use futures::stream::SplitSink;
use futures::StreamExt as _;
use string_cache::DefaultAtom as Atom;
use url::Url;

use crate::proto::message::ClientMessage;
use crate::proto::subscribe::ChannelType;
use crate::proto::subscribe::Subscribe;
use crate::proto::subscribe::Unsubscribe;
use crate::proto::WsCommand;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

/// Interval between automatic reconnections.
const RECONNECT_INTERVAL: Duration = Duration::from_secs(30 * 24 * 60 * 60);

pub struct Websocket {
    api_client: awc::Client,
    ws_url: Url,
    buffer: Option<Vec<u8>>,
    inner: Option<InnerSocket>,
    tx: mpsc::UnboundedSender<ClientMessage>,
    channels: HashMap<ChannelType, HashSet<Atom>>,
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
                self.handle_raw_message(&msg, ctx);
            }
            ws::Frame::Close(_) => {
                ctx.stop();
            }
            ws::Frame::Continuation(frame) => {
                self.handle_continuation(frame, ctx);
            }
        }
    }
}

impl actix::io::WriteHandler<ws::ProtocolError> for Websocket {}

impl Handler<WsCommand> for Websocket {
    type Result = ();

    fn handle(&mut self, cmd: WsCommand, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&cmd).expect("json encode");
        log::debug!("Sending to server: `{}`", msg);
        if let Err(_) = self.inner_mut().sink.write(ws::Message::Text(msg.into())) {
            ctx.stop();
        }

        match cmd {
            WsCommand::Subscribe(Subscribe {
                product_ids,
                channels,
            }) => {
                for channel in channels {
                    let entry = self.channels.entry(channel).or_default();
                    for product_id in &product_ids {
                        entry.insert(product_id.clone());
                    }
                }
            }
            WsCommand::Unsubscribe(Unsubscribe {
                product_ids,
                channels,
            }) => {
                for channel in channels {
                    self.channels.entry(channel).and_modify(|set| {
                        for product_id in &product_ids {
                            set.remove(product_id);
                        }
                    });
                }
            }
        };
    }
}

/// reconnect message
#[derive(actix::Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct ReconnectSocket;

impl Handler<ReconnectSocket> for Websocket {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _: ReconnectSocket, _: &mut Self::Context) -> Self::Result {
        let api_client = self.api_client.clone();
        let ws_url = self.ws_url.clone();

        let fut = async move { api_client.ws(ws_url.as_str()).connect().await };
        let fut = fut.into_actor(self).then(|res, act, ctx| {
            let (resp, connection) = match res {
                Ok((resp, connection)) => (resp, connection),
                Err(err) => {
                    log::error!("Socket connection was not initialized: {err}");
                    ctx.stop();
                    return fut::ready(());
                }
            };
            log::debug!("Websocket response: {resp:?}");
            let (sink, stream) = connection.split();

            ctx.add_stream(stream);
            act.inner = Some(InnerSocket {
                sink: SinkWrite::new(sink, ctx),
                hb: Instant::now(),
            });

            // Resubscribe to previous subscriptions.
            let old_subscriptions = std::mem::take(&mut act.channels);
            for (ty, product_ids) in old_subscriptions {
                let subscribe = Subscribe {
                    product_ids: product_ids.into_iter().collect(),
                    channels: vec![ty],
                };
                ctx.notify(WsCommand::Subscribe(subscribe));
            }

            fut::ready(())
        });
        Box::pin(fut)
    }
}

impl Websocket {
    pub fn new(client: awc::Client, url: Url, tx: mpsc::UnboundedSender<ClientMessage>) -> Self {
        Self {
            api_client: client,
            ws_url: url,
            tx,
            channels: HashMap::new(),
            buffer: None,
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

    /// Handles Frame::Continue message.
    fn handle_continuation(&mut self, msg: FrameItem, ctx: &mut Context<Self>) {
        match msg {
            FrameItem::FirstText(bytes) => {
                if self.buffer.is_some() {
                    // This means that there's already another continuous
                    // message.
                    log::error!("unexpected `FirstText` frame");
                    ctx.stop();
                    return;
                }
                self.buffer = Some(bytes.to_vec())
            }
            FrameItem::FirstBinary(_) => {
                log::warn!("unexpected binary message (ignored)");
            }
            FrameItem::Continue(bytes) => {
                match self.buffer.as_mut() {
                    Some(buffer) => buffer.extend_from_slice(&bytes),
                    None => {
                        log::error!("unexpected `Continue` frame");
                        // Stop to avoid data corruption.
                        ctx.stop();
                    }
                }
            }
            FrameItem::Last(bytes) => {
                let buffer = self.buffer.take();
                match buffer {
                    None => {
                        // No continuous message in progress.
                        log::error!("unexpected `Last` frame");
                        ctx.stop();
                    }
                    Some(mut buffer) => {
                        buffer.extend_from_slice(&bytes);
                        self.handle_raw_message(&buffer, ctx);
                    }
                }
            }
        }
    }

    fn handle_raw_message(&mut self, msg: &[u8], ctx: &mut Context<Self>) {
        let res = serde_json::from_slice::<ClientMessage>(&msg);
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

        if let Err(e) = self.tx.unbounded_send(event) {
            log::warn!("Failed to notify downstream: {:?}", e);
            ctx.stop()
        }
    }

    // fn handle_system_msg(&mut self, ev: SystemEvent, ctx: &mut Context<Self>) {
    //     match ev {
    //         SystemEvent::SubscriptionSucceeded { channel } => {
    //             let subscription = WsSubscription::from(channel);
    //             if !self.channels.contains_key(&subscription) {
    //                 log::warn!(
    //                     "Successfully subscribed to {:?}. But it was \
    //                      not found in list of active subscriptions",
    //                     subscription,
    //                 );
    //             }
    //             self.channels.insert(subscription, true);
    //         }
    //         SystemEvent::Error { channel } => {
    //             log::error!("Websocket Channel({}) returned error", channel);
    //         }
    //     }
    // }
}
