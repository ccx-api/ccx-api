use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use futures::channel::mpsc;
use futures::lock::Mutex;
use futures::prelude::*;
use tokio::time::sleep;

use super::super::BucketName;
use super::super::PrimeTaskBuilder;
use super::super::Queue;
use super::super::RateLimiterBucket;
use super::super::TaskCosts;
use super::super::TaskMessage;
use crate::CoinbaseResult;
use crate::LibError;
use crate::client::CoinbasePrimeSigner;
use crate::client::PrimeRequestBuilder;

#[derive(Clone)]
pub(crate) struct PrimeRateLimiter {
    buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
    queue: Arc<Mutex<Queue>>,
}

impl PrimeRateLimiter {
    pub(in super::super) fn new(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        tasks_tx: mpsc::UnboundedSender<TaskMessage>,
        queue: Arc<Mutex<Queue>>,
    ) -> Self {
        PrimeRateLimiter {
            buckets,
            tasks_tx,
            queue,
        }
    }

    pub fn task<S>(&self, builder: PrimeRequestBuilder<S>) -> PrimeTaskBuilder<S>
    where
        S: CoinbasePrimeSigner + Unpin,
    {
        PrimeTaskBuilder::new(0, TaskCosts::new(), builder, self.tasks_tx.clone())
    }

    pub(in super::super) fn recv(&self, mut rx: mpsc::UnboundedReceiver<TaskMessage>) {
        let buckets = self.buckets.clone();
        let queue = self.queue.clone();
        tokio::spawn(async move {
            while let Some(task_message) = rx.next().await {
                let is_first_task = queue.lock().await.add(task_message).is_first();
                if is_first_task {
                    Self::handler(buckets.clone(), queue.clone()).await;
                }
            }
        });
    }

    pub(super) async fn handler(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        queue: Arc<Mutex<Queue>>,
    ) {
        tokio::spawn(async move {
            loop {
                let TaskMessage {
                    priority,
                    costs,
                    tx,
                } = match queue.lock().await.next() {
                    Some(task) => task,
                    None => {
                        log::debug!("RateLimiter: stop queue handler (queue is empty)");
                        break;
                    }
                };
                log::debug!("RateLimiter: received task with priority {}", priority);

                let buckets = buckets.clone();
                let res = async move {
                    if let Some(dur) = Self::timeout(buckets.clone(), &costs).await? {
                        log::debug!("RateLimiter: sleep for {:?}", dur);
                        sleep(dur).await;
                    }
                    Self::set_costs(buckets, &costs).await?;
                    Ok(())
                }
                .await;

                log::debug!("RateLimiter: completed task with priority {}", priority);
                let _ = tx.send(res);
            }
        });
    }

    pub(super) async fn timeout(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &TaskCosts,
    ) -> CoinbaseResult<Option<Duration>> {
        let mut timeout = Duration::default();

        for (name, cost) in costs {
            let mut bucket = match buckets.get(name) {
                Some(bucket) => bucket.lock().await,
                None => Err(LibError::other(format!(
                    "RateLimiter: undefined bucket {}",
                    name
                )))?,
            };

            let delay = bucket.delay.duration_since(Instant::now());
            if !delay.is_zero() {
                log::debug!("RateLimiter: bucket {} :: Delayed start {:?}", name, delay);
                timeout = delay;
                continue;
            }

            bucket.update_state();
            let new_amount = bucket.amount + cost;
            log::debug!(
                "RateLimiter: bucket {} :: Task cost {}; prev amount {}; bucket limit: {};",
                name,
                cost,
                bucket.amount,
                bucket.limit
            );

            if new_amount > bucket.limit {
                let bucket_timeout = bucket.get_timeout();
                log::debug!("RateLimiter: bucket {} :: Limit has been reached", name);

                if bucket_timeout > timeout {
                    log::debug!(
                        "RateLimiter: bucket {} :: Need sleep {:?}.",
                        name,
                        bucket_timeout
                    );
                    timeout = bucket_timeout;
                }
            }
        }

        Ok((!timeout.is_zero()).then_some(timeout))
    }

    pub(super) async fn set_costs(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &TaskCosts,
    ) -> CoinbaseResult<()> {
        for (name, cost) in costs {
            let mut bucket = match buckets.get(name) {
                Some(bucket) => bucket.lock().await,
                None => Err(LibError::other(format!(
                    "RateLimiter: undefined bucket {}",
                    name
                )))?,
            };

            bucket.update_state();
            bucket.amount += cost;

            log::debug!(
                "RateLimiter: bucket {} :: New amount {}; bucket limit: {}",
                name,
                bucket.amount,
                bucket.limit
            );
        }

        Ok(())
    }
}
