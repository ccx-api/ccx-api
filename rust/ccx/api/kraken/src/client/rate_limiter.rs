use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use actix::clock::sleep;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::lock::Mutex;
use futures::prelude::*;
use futures::task::Context;
use futures::task::Poll;

use super::KrakenSigner;
use super::RequestBuilder;
use crate::KrakenApiResult;

type TaskCosts = HashMap<String, u64>;

#[derive(Default)]
pub(crate) struct RateLimiterBuilder {
    buckets: HashMap<String, RateLimiterBucket>,
}

impl RateLimiterBuilder {
    pub fn bucket(mut self, key: impl Into<String>, bucket: RateLimiterBucket) -> Self {
        match self.buckets.entry(key.into()) {
            Entry::Occupied(mut e) => *e.get_mut() = bucket,
            Entry::Vacant(e) => {
                e.insert(bucket);
            }
        }
        self
    }

    pub fn start(self) -> RateLimiter {
        let (queue_tx, queue_rx) = mpsc::unbounded::<TaskMessage>();
        let rate_limiter = RateLimiter {
            buckets: Arc::new(
                self.buckets
                    .into_iter()
                    .map(|(k, v)| (k, Mutex::new(v.into())))
                    .collect(),
            ),
            queue_tx,
            // queue: Arc::new(Mutex::new(Vec::new())),
        };
        rate_limiter.recv(queue_rx);
        rate_limiter
    }
}

#[derive(Clone)]
pub(crate) struct RateLimiter {
    buckets: Arc<HashMap<String, Mutex<RateLimiterBucket>>>,
    queue_tx: mpsc::UnboundedSender<TaskMessage>,
    // queue: Arc<Mutex<Vec<TaskMessage>>>,
}

impl RateLimiter {
    pub fn task<S>(&self, builder: RequestBuilder<S>) -> TaskBuilder<S>
    where
        S: KrakenSigner + Unpin,
    {
        TaskBuilder {
            req_builder: builder,
            costs: TaskCosts::new(),
            queue_tx: self.queue_tx.clone(),
        }
    }

    fn recv(&self, mut rx: mpsc::UnboundedReceiver<TaskMessage>) {
        let buckets = self.buckets.clone();
        actix_rt::spawn(async move {
            while let Some(TaskMessage { costs, task_tx }) = rx.next().await {
                if let Some(timeout) = Self::timeout(buckets.clone(), costs).await {
                    println!("XXX: recv SLEEP {:?}", timeout);
                    let t = Instant::now();
                    println!("XXX time before {:?}", t);
                    sleep(timeout).await;
                    println!("XXX time after {:?}", Instant::now() - t);
                }

                println!("XXX: recv task send");
                let _ = task_tx.send(());
            }
        });
    }

    async fn timeout(
        buckets: Arc<HashMap<String, Mutex<RateLimiterBucket>>>,
        costs: TaskCosts,
    ) -> Option<Duration> {
        let mut timeout = Duration::default();

        for (name, cost) in &costs {
            if let Some(bucket) = buckets.get(name) {
                let mut bucket = bucket.lock().await;

                // TODO: сделать delay

                let elapsed = Instant::now().duration_since(bucket.instant);
                if elapsed > bucket.interval {
                    bucket.instant = Instant::now();
                    bucket.amount = 0;
                }
                bucket.amount += cost;

                if bucket.amount > bucket.limit {
                    let elapsed = Instant::now().duration_since(bucket.instant);
                    let bucket_timeout = bucket.interval - elapsed;

                    if bucket_timeout > timeout {
                        timeout = bucket_timeout;
                    }
                }
            }
        }

        (!timeout.is_zero()).then(|| timeout)
    }
}

pub(crate) struct RateLimiterBucket {
    delay: Duration,
    interval: Duration,
    instant: Instant,
    limit: u64,
    amount: u64,
}

impl Default for RateLimiterBucket {
    fn default() -> Self {
        Self {
            delay: Duration::default(),
            interval: Duration::default(),
            instant: Instant::now(),
            limit: 0,
            amount: 0,
        }
    }
}

impl RateLimiterBucket {
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = limit;
        self
    }
}

pub(crate) struct TaskBuilder<S>
where
    S: KrakenSigner + Unpin + 'static,
{
    req_builder: RequestBuilder<S>,
    costs: TaskCosts,
    queue_tx: mpsc::UnboundedSender<TaskMessage>,
}

impl<S> TaskBuilder<S>
where
    S: KrakenSigner + Unpin + 'static,
{
    pub fn cost(mut self, key: impl Into<String>, weight: u64) -> Self {
        self.costs
            .entry(key.into())
            .and_modify(|e| *e = weight)
            .or_insert(weight);
        self
    }

    pub fn send<V>(self) -> Task<V>
    where
        V: serde::de::DeserializeOwned + Debug,
    {
        let costs = self.costs.clone();
        let req_builder = self.req_builder;
        let mut queue_tx = self.queue_tx.clone();
        let fut = async move {
            let (task_tx, task_rx) = oneshot::channel::<()>();
            let _ = queue_tx.send(TaskMessage { costs, task_tx }).await;
            let _ = task_rx.await;
            req_builder.send::<V>().await
        };

        Task {
            fut: fut.boxed_local(),
            costs: self.costs,
        }
    }
}

pub(crate) struct Task<V>
where
    V: serde::de::DeserializeOwned + Debug,
{
    fut: Pin<Box<dyn Future<Output = KrakenApiResult<V>>>>,
    costs: TaskCosts,
}

impl<V> Task<V>
where
    V: serde::de::DeserializeOwned + Debug,
{
    pub fn metadata(&self) -> TaskMetadata {
        TaskMetadata {
            costs: self.costs.clone(),
        }
    }
}

impl<V> Future for Task<V>
where
    V: serde::de::DeserializeOwned + Debug,
{
    type Output = KrakenApiResult<V>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.poll_unpin(cx)
    }
}

pub(crate) struct TaskMetadata {
    costs: TaskCosts,
}

struct TaskMessage {
    costs: TaskCosts,
    task_tx: oneshot::Sender<()>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ApiCred;
    use crate::Proxy;
    use crate::SpotApi;

    use crate::api::spot::AssetInfoResponse;

    pub static CCX_KRAKEN_API_PREFIX: &str = "CCX_KRAKEN_API";

    #[actix_rt::test]
    async fn test_queue() {
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let spot_api = SpotApi::new(ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX), proxy);

        let mut rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "key1",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(1))
                    .limit(1),
            )
            .bucket(
                "key2",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(10))
                    .limit(2),
            )
            .start();

        for _ in 1..10 {
            let s = rate_limiter
                .task(
                    spot_api
                        .client
                        .get("/0/public/Assets")
                        .unwrap()
                        .try_query_arg("pairs", &None::<&str>)
                        .unwrap()
                        .try_query_arg("info", &None::<&str>)
                        .unwrap(),
                )
                .cost("key1", 1)
                .cost("key2", 1)
                .send::<AssetInfoResponse>()
                .await;

            println!("XXX RES!");
        }

        // let res: Task<AssetInfoResponse> = rate_limiter
        //     .task(
        //         spot_api
        //             .client
        //             .get("/0/public/Assets")
        //             .unwrap()
        //             .try_query_arg("pairs", &None::<&str>)
        //             .unwrap()
        //             .try_query_arg("info", &None::<&str>)
        //             .unwrap(),
        //     )
        //     .cost("key1", 1)
        //     .cost("key2", 1)
        //     .send();
        // println!("task costs: {:?}", res.metadata());
        // println!("task await {:?}", res.await);

        // println!("queue {:?}", rate_limiter.queue.lock().await.len());
    }
}
