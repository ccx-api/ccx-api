use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use actix::clock::sleep;
use futures::channel::mpsc;
use futures::lock::Mutex;
use futures::prelude::*;

use super::super::BucketName;
use super::super::Queue;
use super::super::RateLimiterBucket;
use super::super::TaskCosts;
use super::super::TaskMessage;
use super::super::TradeTaskBuilder;
use crate::client::CoinbaseTradeSigner;
use crate::client::TradeRequestBuilder;
use crate::CoinbaseResult;
use crate::LibError;

#[derive(Clone)]
pub(crate) struct TradeRateLimiter {
    buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
    queue: Arc<Mutex<Queue>>,
}

impl TradeRateLimiter {
    pub(in super::super) fn new(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        tasks_tx: mpsc::UnboundedSender<TaskMessage>,
        queue: Arc<Mutex<Queue>>,
    ) -> Self {
        TradeRateLimiter {
            buckets,
            tasks_tx,
            queue,
        }
    }

    pub fn task<S>(&self, builder: TradeRequestBuilder<S>) -> TradeTaskBuilder<S>
    where
        S: CoinbaseTradeSigner + Unpin,
    {
        TradeTaskBuilder::new(0, TaskCosts::new(), builder, self.tasks_tx.clone())
    }

    pub(in super::super) fn recv(&self, mut rx: mpsc::UnboundedReceiver<TaskMessage>) {
        let buckets = self.buckets.clone();
        let queue = self.queue.clone();
        actix_rt::spawn(async move {
            while let Some(task_message) = rx.next().await {
                let is_first_task = queue.lock().await.add(task_message).is_first();
                if is_first_task {
                    Self::handler(buckets.clone(), queue.clone()).await;
                }
            }
        });
    }

    pub(super) async fn handler<'a>(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        queue: Arc<Mutex<Queue>>,
    ) {
        let buckets = buckets.clone();
        let queue = queue.clone();
        actix_rt::spawn(async move {
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

    pub(super) async fn timeout<'a>(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &'a TaskCosts,
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

        Ok((!timeout.is_zero()).then(|| timeout))
    }

    pub(super) async fn set_costs<'a>(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &'a TaskCosts,
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
