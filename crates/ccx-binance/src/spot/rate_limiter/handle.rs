use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::SinkExt;

use crate::spot::rate_limiter::actor::RateLimiterActor;
use crate::spot::rate_limiter::error::RateLimiterError;
use crate::spot::rate_limiter::types::RateLimiterMessage;
use crate::spot::rate_limiter::types::Task;
use crate::spot::rate_limiter::types::TaskCosts;

#[derive(Clone)]
pub struct RateLimiter {
    tx: mpsc::Sender<RateLimiterMessage>,
}

impl RateLimiter {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel(8);
        let actor = RateLimiterActor::new_prepared();
        tokio::spawn(async move {
            actor.run(rx).await;
        });
        RateLimiter { tx }
    }

    pub async fn enqueue(
        &mut self,
        priority: u8,
        costs: TaskCosts,
    ) -> Result<(), RateLimiterError> {
        let (tx, rx) = oneshot::channel();
        let message = RateLimiterMessage::Enqueue(Task {
            priority,
            costs,
            tx,
        });
        let () = self.tx.send(message).await?;
        let () = rx.await?;
        Ok(())
    }
}
