use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use actix::clock::sleep;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::lock::Mutex;
use futures::prelude::*;
use futures::task::Context;
use futures::task::Poll;

use super::BinanceSigner;
use super::RequestBuilder;
use crate::BinanceResult;
use crate::LibError;

type BucketName = Cow<'static, str>;
type TaskCosts = HashMap<BucketName, u32>;
type TaskMessageResult = BinanceResult<()>;

struct TaskMessage {
    priority: u8,
    costs: TaskCosts,
    tx: oneshot::Sender<TaskMessageResult>,
}

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
        let rate_limiter = RateLimiter {
            buckets: Arc::new(buckets),
            tasks_tx,
            queue: Arc::new(Mutex::new(Queue::new())),
        };
        rate_limiter.recv(tasks_rx);
        rate_limiter
    }
}

#[derive(Clone)]
pub(crate) struct RateLimiter {
    buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
    queue: Arc<Mutex<Queue>>,
}

impl RateLimiter {
    pub fn task<S>(&self, builder: RequestBuilder<S>) -> TaskBuilder<S>
    where
        S: BinanceSigner + Unpin,
    {
        TaskBuilder {
            priority: 0,
            costs: TaskCosts::new(),
            req_builder: builder,
            tasks_tx: self.tasks_tx.clone(),
        }
    }

    fn recv(&self, mut rx: mpsc::UnboundedReceiver<TaskMessage>) {
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

    async fn handler(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        queue: Arc<Mutex<Queue>>,
    ) {
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

    async fn timeout(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &TaskCosts,
    ) -> BinanceResult<Option<Duration>> {
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

    async fn set_costs(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &TaskCosts,
    ) -> BinanceResult<()> {
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

pub(crate) struct RateLimiterBucket {
    time_instant: Instant,
    delay: Instant,
    interval: Duration,
    limit: u32,
    amount: u32,
}

impl Default for RateLimiterBucket {
    fn default() -> Self {
        Self {
            time_instant: Instant::now(),
            delay: Instant::now(),
            interval: Duration::default(),
            limit: 0,
            amount: 0,
        }
    }
}

impl RateLimiterBucket {
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Instant::now() + delay;
        self
    }

    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    fn update_state(&mut self) {
        let elapsed = Instant::now().duration_since(self.time_instant);
        if elapsed > self.interval {
            self.time_instant = Instant::now();
            self.amount = 0;
        }
    }

    fn get_timeout(&self) -> Duration {
        let elapsed = Instant::now().duration_since(self.time_instant);
        self.interval - elapsed
    }
}

struct Queue {
    inner: VecDeque<TaskMessage>,
}

impl Queue {
    fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    fn add(&mut self, msg: TaskMessage) -> &Self {
        let priority = msg.priority;
        self.inner.push_back(msg);

        if priority > 0 {
            self.inner
                .make_contiguous()
                .sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        self
    }

    fn is_first(&self) -> bool {
        self.inner.len() == 1
    }
}

impl Iterator for Queue {
    type Item = TaskMessage;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}

pub(crate) struct TaskBuilder<S>
where
    S: BinanceSigner + Unpin + 'static,
{
    priority: u8,
    costs: TaskCosts,
    req_builder: RequestBuilder<S>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
}

impl<S> TaskBuilder<S>
where
    S: BinanceSigner + Unpin + 'static,
{
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn cost(mut self, key: impl Into<BucketName>, weight: u32) -> Self {
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
        let priority = self.priority;
        let costs = self.costs.clone();
        let req_builder = self.req_builder;
        let mut tasks_tx = self.tasks_tx.clone();

        let fut = async move {
            let (tx, rx) = oneshot::channel::<TaskMessageResult>();
            tasks_tx
                .send(TaskMessage {
                    priority,
                    costs,
                    tx,
                })
                .await
                .map_err(|_| LibError::other("RateLimiter: task channel was dropped"))?;
            rx.await
                .map_err(|_| LibError::other("RateLimiter: task channel was dropped"))?
                .map_err(|e| {
                    log::error!("RateLimiter: task err. {:?}", e);
                    e
                })?;

            req_builder.send::<V>().await
        };

        Task {
            fut: fut.boxed_local(),
            costs: self.costs,
        }
    }
}

pub struct Task<V>
where
    V: serde::de::DeserializeOwned + Debug,
{
    fut: Pin<Box<dyn Future<Output = BinanceResult<V>>>>,
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
    type Output = BinanceResult<V>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.poll_unpin(cx)
    }
}

#[derive(Debug)]
pub struct TaskMetadata {
    pub costs: TaskCosts,
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicU8;
    use std::sync::atomic::Ordering;

    use super::*;
    use crate::api::spot::ServerTime;
    use crate::ApiCred;
    use crate::Proxy;
    use crate::SpotApi;

    pub static CCX_BINANCE_API_PREFIX: &str = "CCX_BINANCE_API";

    #[actix_rt::test]
    async fn test_rate_limiter_queue() {
        let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
        let spot_api = SpotApi::new(
            ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
            true,
            proxy,
        );

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "interval_1__limit_1",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(1))
                    .limit(1),
            )
            .bucket(
                "interval_10__limit_2",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(10))
                    .limit(2),
            )
            .start();

        let instant = Instant::now();
        for _i in 1..=8 {
            let task_res = rate_limiter
                .task(spot_api.client.get("/api/v3/time").unwrap())
                .cost("interval_1__limit_1", 1)
                .cost("interval_10__limit_2", 1)
                .send::<ServerTime>()
                .await;
            println!("TASK {:?}", task_res);

            assert!(task_res.is_ok());
        }

        assert!(instant.elapsed() >= Duration::from_secs(30));
    }

    #[actix_rt::test]
    async fn test_rate_limiter_metadata() {
        let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
        let spot_api = SpotApi::new(
            ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
            true,
            proxy,
        );

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "interval_1__limit_1",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(1))
                    .limit(1),
            )
            .bucket(
                "interval_10__limit_2",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(10))
                    .limit(2),
            )
            .start();

        for _i in 1..=8 {
            let task = rate_limiter
                .task(spot_api.client.get("/api/v3/time").unwrap())
                .cost("interval_1__limit_1", 1)
                .cost("interval_10__limit_2", 1)
                .send::<ServerTime>();

            assert_eq!(task.metadata().costs.get("interval_1__limit_1"), Some(&1));
            assert_eq!(task.metadata().costs.get("interval_10__limit_2"), Some(&1));
        }
    }

    #[actix_rt::test]
    async fn test_rate_limiter_delay() {
        let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
        let spot_api = SpotApi::new(
            ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
            true,
            proxy,
        );

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "delay_10__interval_1__limit_1",
                RateLimiterBucket::default()
                    .delay(Duration::from_secs(10))
                    .interval(Duration::from_secs(10))
                    .limit(1),
            )
            .start();

        let instant = Instant::now();
        for _i in 1..=2 {
            let task_res = rate_limiter
                .task(spot_api.client.get("/api/v3/time").unwrap())
                .cost("delay_10__interval_1__limit_1", 1)
                .send::<ServerTime>()
                .await;

            assert!(task_res.is_ok());
        }

        assert!(instant.elapsed() >= Duration::from_secs(20));
    }

    #[actix_rt::test]
    async fn test_rate_limiter_wrong_bucket() {
        let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
        let spot_api = SpotApi::new(
            ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
            true,
            proxy,
        );

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "delay_10__interval_1__limit_1",
                RateLimiterBucket::default()
                    .delay(Duration::from_secs(10))
                    .interval(Duration::from_secs(10))
                    .limit(1),
            )
            .start();

        let task_res = rate_limiter
            .task(spot_api.client.get("/api/v3/time").unwrap())
            .cost("interval_1__limit_1", 1)
            .send::<ServerTime>()
            .await;
        assert!(task_res.is_err())
    }

    #[actix_rt::test]
    async fn test_rate_limiter_priority() {
        let proxy = Proxy::from_env_with_prefix(CCX_BINANCE_API_PREFIX);
        let spot_api = SpotApi::new(
            ApiCred::from_env_with_prefix(CCX_BINANCE_API_PREFIX),
            true,
            proxy,
        );

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "interval_3__limit_5",
                RateLimiterBucket::default()
                    .interval(Duration::from_secs(1))
                    .limit(1),
            )
            .start();

        let instant = Instant::now();
        let counter = Arc::new(AtomicU8::new(0));
        let position = Arc::new(AtomicU8::new(0));
        {
            let counter = counter.clone();
            let position = position.clone();
            let rate_limiter = rate_limiter.clone();
            let spot_api = spot_api.clone();
            actix::spawn(async move {
                while counter.load(Ordering::SeqCst) < 6 {
                    sleep(Duration::from_millis(10)).await;
                }

                let _task_res = rate_limiter
                    .task(spot_api.client.get("/api/v3/time").unwrap())
                    .cost("interval_3__limit_5", 1)
                    .priority(1)
                    .send::<ServerTime>()
                    .await;
                println!(
                    "Time now: {:?}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );
                let current = counter.fetch_add(1, Ordering::SeqCst) + 1;
                position.store(current, Ordering::SeqCst);
                println!("PRIORITY POS: {}", current);
            });
        }

        for _ in 1..10 {
            let counter = counter.clone();
            let rate_limiter = rate_limiter.clone();
            let spot_api = spot_api.clone();
            actix::spawn(async move {
                let _task_res = rate_limiter
                    .task(spot_api.client.get("/api/v3/time").unwrap())
                    .cost("interval_3__limit_5", 1)
                    .send::<ServerTime>()
                    .await;
                println!(
                    "Time now: {:?}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );
                let current = counter.fetch_add(1, Ordering::SeqCst) + 1;
                println!("TASK POS: {}", current);
            });
        }

        while counter.load(Ordering::SeqCst) < 10 {
            sleep(Duration::from_millis(100)).await;
        }

        assert!((7..=8).contains(&position.load(Ordering::SeqCst)));
        assert!(instant.elapsed() >= Duration::from_secs(9));
    }
}
