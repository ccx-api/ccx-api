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
use string_cache::DefaultAtom as Atom;

use crate::proto::message::ClientMessage;
use crate::proto::subscribe::ChannelType;
use crate::proto::subscribe::Subscribe;
use crate::proto::subscribe::Unsubscribe;
use crate::proto::WsCommand;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

type SocketSink = SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>;

pub struct Websocket {
    buffer: Option<Vec<u8>>,
    inner: InnerSocket,
    tx: mpsc::UnboundedSender<ClientMessage>,
    channels: HashMap<ChannelType, HashSet<Atom>>,
}

struct InnerSocket {
    sink: SocketSink,
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
                self.inner.hb = Instant::now();
                if let Err(_msg) = self.inner.sink.write(ws::Message::Pong(msg)) {
                    log::warn!("Failed to send Pong. Disconnecting.");
                    ctx.stop()
                }
            }
            ws::Frame::Pong(_) => {
                self.inner.hb = Instant::now();
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
        if let Err(_) = self.inner.sink.write(ws::Message::Text(msg.into())) {
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

impl Websocket {
    pub fn new(sink: SocketSink, tx: mpsc::UnboundedSender<ClientMessage>) -> Self {
        Self {
            tx,
            channels: HashMap::new(),
            buffer: None,
            inner: InnerSocket {
                sink,
                hb: Instant::now(),
            },
        }
    }

    fn hb(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.inner.hb) > CLIENT_TIMEOUT {
                log::warn!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            if let Err(_msg) = act.inner.sink.write(ws::Message::Ping("".into())) {
                log::warn!("Websocket client failed to send ping, stopping!");
                ctx.stop()
            };
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
