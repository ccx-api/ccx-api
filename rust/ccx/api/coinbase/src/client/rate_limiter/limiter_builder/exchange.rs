use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use futures::channel::mpsc;
use futures::lock::Mutex;

use super::super::BucketName;
use super::super::queue::Queue;
use super::super::task_message::TaskMessage;
use crate::client::ExchangeRateLimiter;
use crate::client::RateLimiterBucket;

#[derive(Default)]
pub(crate) struct ExchangeRateLimiterBuilder {
    buckets: HashMap<BucketName, RateLimiterBucket>,
}

impl ExchangeRateLimiterBuilder {
    pub fn bucket(mut self, key: impl Into<BucketName>, bucket: RateLimiterBucket) -> Self {
        match self.buckets.entry(key.into()) {
            Entry::Occupied(mut e) => *e.get_mut() = bucket,
            Entry::Vacant(e) => {
                e.insert(bucket);
            }
        }
        self
    }

    pub fn start(self) -> ExchangeRateLimiter {
        let (tasks_tx, tasks_rx) = mpsc::unbounded::<TaskMessage>();
        let buckets = self
            .buckets
            .into_iter()
            .map(|(k, v)| (k, Mutex::new(v.into())))
            .collect();

        let rate_limiter = ExchangeRateLimiter::new(Arc::new(buckets), tasks_tx, Arc::new(Mutex::new(Queue::new())));
        rate_limiter.recv(tasks_rx);
        rate_limiter
    }
}
