use futures::channel::mpsc;
use futures::channel::oneshot;

#[derive(Debug, thiserror::Error)]
pub enum RateLimiterError {
    #[error("Send: {0}")]
    Send(#[from] mpsc::SendError),
    #[error("Await: {0}")]
    Await(#[from] oneshot::Canceled),
}
