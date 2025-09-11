use actix::Context;
use actix::Handler;

use crate::client::websocket::WebSocketActor;
use crate::client::websocket::implement::message::Die;
use crate::client::websocket::implement::message::WsCommand;
use crate::error::LibResult;
use crate::types::WsRequest;

impl Handler<Die> for WebSocketActor {
    type Result = ();

    fn handle(&mut self, _: Die, ctx: &mut Context<Self>) -> Self::Result {
        println!("handle Die and restarting for ws connect/reconnect");
        self.stop_context(ctx);
    }
}

impl Handler<WsCommand> for WebSocketActor {
    type Result = LibResult<()>;

    fn handle(&mut self, WsCommand(msg): WsCommand, ctx: &mut Context<Self>) -> Self::Result {
        println!("handle ws commend :: {}", msg);
        self.send(msg, ctx)
    }
}

impl Handler<WsRequest> for WebSocketActor {
    type Result = LibResult<()>;

    fn handle(&mut self, msg: WsRequest, ctx: &mut Context<Self>) -> Self::Result {
        println!("handle ws request :: {:?}", msg);
        let message = serde_json::to_string(&msg)?;
        self.send(message, ctx)
    }
}
