use std::hash::Hash;

use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::oneshot;

use super::actor::{RateLimiterActor, RateLimiterBucket};
use super::error::RateLimiterError;
use super::types::RateLimiterMessage;
use super::types::Task;
use super::types::TaskCosts;

#[derive(Clone)]
pub struct RateLimiter<RateLimitType: 'static> {
    tx: mpsc::Sender<RateLimiterMessage<RateLimitType>>,
}

impl<RateLimitType> RateLimiter<RateLimitType>
where
    RateLimitType: std::fmt::Debug + Hash + Eq + 'static,
    RateLimitType: Send + Sync,
{
    pub fn spawn<BucketInit>(bucket_init: BucketInit) -> Self
    where
        BucketInit: Fn(&RateLimitType) -> Vec<RateLimiterBucket>,
        BucketInit: 'static + Send,
    {
        let (tx, rx) = mpsc::channel(8);
        let actor = RateLimiterActor::with_bucket_initializer(bucket_init);
        tokio::spawn(async move {
            actor.run(rx).await;
        });
        RateLimiter { tx }
    }

    pub async fn enqueue(
        &mut self,
        priority: u8,
        costs: TaskCosts<RateLimitType>,
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
