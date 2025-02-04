use crate::error::LibResult;

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Die;

#[derive(actix::Message)]
#[rtype(result = "LibResult<()>")]
pub struct WsCommand(pub String);
