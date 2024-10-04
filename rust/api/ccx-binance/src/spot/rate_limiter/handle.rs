use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::SinkExt;

use crate::spot::rate_limiter::error::RateLimiterError;
use crate::spot::rate_limiter::types::Task;
use crate::spot::rate_limiter::types::TaskMessage;

#[derive(Clone)]
pub struct RateLimiter {
    tx: mpsc::UnboundedSender<TaskMessage>,
}

impl RateLimiter {
    pub fn new(tx: mpsc::UnboundedSender<TaskMessage>) -> Self {
        RateLimiter { tx }
    }

    pub async fn enqueue(
        &mut self,
        priority: u8,
        costs: &'static [(&'static str, u32)],
    ) -> Result<(), RateLimiterError> {
        let (tx, rx) = oneshot::channel();
        let message = TaskMessage::Enqueue(Task {
            priority,
            costs,
            tx,
        });
        let () = self.tx.send(message).await?;
        let () = rx.await?;
        Ok(())
    }
}
