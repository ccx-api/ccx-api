use futures::channel::mpsc;
use futures::channel::oneshot;

#[derive(Debug, derive_more::Display, derive_more::From, derive_more::Error)]
pub enum RateLimiterError {
    #[display("Send: {_0}")]
    Send(mpsc::SendError),
    #[display("Await: {_0}")]
    Await(oneshot::Canceled),
}
