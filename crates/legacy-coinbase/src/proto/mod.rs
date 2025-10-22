use serde::Serialize;
use subscribe::Subscribe;
use subscribe::Unsubscribe;

pub mod auction;
pub mod message;
pub mod subscribe;
pub mod ticker;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum WsCommand {
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
}
