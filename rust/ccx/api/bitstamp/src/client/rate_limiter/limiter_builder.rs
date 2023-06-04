use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use futures::channel::mpsc;
use futures::lock::Mutex;

use super::queue::Queue;
use super::task_message::TaskMessage;
use super::BucketName;
use crate::client::RateLimiter;
use crate::client::RateLimiterBucket;

#[derive(Default)]
pub(crate) struct RateLimiterBuilder {
    buckets: HashMap<BucketName, RateLimiterBucket>,
}

impl RateLimiterBuilder {
    pub fn bucket(mut self, key: impl Into<BucketName>, bucket: RateLimiterBucket) -> Self {
        match self.buckets.entry(key.into()) {
            Entry::Occupied(mut e) => *e.get_mut() = bucket,
            Entry::Vacant(e) => {
                e.insert(bucket);
            }
        }
        self
    }

    pub fn start(self) -> RateLimiter {
        let (tasks_tx, tasks_rx) = mpsc::unbounded::<TaskMessage>();
        let buckets = self
            .buckets
            .into_iter()
            .map(|(k, v)| (k, Mutex::new(v)))
            .collect();

        let rate_limiter = RateLimiter::new(
            Arc::new(buckets),
            tasks_tx,
            Arc::new(Mutex::new(Queue::new())),
        );
        rate_limiter.recv(tasks_rx);
        rate_limiter
    }
}
