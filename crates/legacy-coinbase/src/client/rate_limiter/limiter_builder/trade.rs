use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Arc;

use futures::channel::mpsc;
use futures::lock::Mutex;

use super::super::BucketName;
use super::super::queue::Queue;
use super::super::task_message::TaskMessage;
use crate::client::RateLimiterBucket;
use crate::client::TradeRateLimiter;

#[derive(Default)]
pub(crate) struct TradeRateLimiterBuilder {
    buckets: HashMap<BucketName, RateLimiterBucket>,
}

impl TradeRateLimiterBuilder {
    pub fn bucket(mut self, key: impl Into<BucketName>, bucket: RateLimiterBucket) -> Self {
        match self.buckets.entry(key.into()) {
            Entry::Occupied(mut e) => *e.get_mut() = bucket,
            Entry::Vacant(e) => {
                e.insert(bucket);
            }
        }
        self
    }

    pub fn start(self) -> TradeRateLimiter {
        let (tasks_tx, tasks_rx) = mpsc::unbounded::<TaskMessage>();
        let buckets = self
            .buckets
            .into_iter()
            .map(|(k, v)| (k, Mutex::new(v)))
            .collect();

        let rate_limiter = TradeRateLimiter::new(
            Arc::new(buckets),
            tasks_tx,
            Arc::new(Mutex::new(Queue::new())),
        );
        rate_limiter.recv(tasks_rx);
        rate_limiter
    }
}
