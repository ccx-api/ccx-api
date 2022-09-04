use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
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
use crate::KrakenResult;
use crate::LibError;

type BucketName = Cow<'static, str>;
type TaskCosts = HashMap<BucketName, u32>;
type TaskMessageResult = KrakenResult<()>;

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
            .map(|(k, v)| (k, Mutex::new(v.into())))
            .collect();

        let rate_limiter = RateLimiter {
            buckets: Arc::new(buckets),
            tasks_tx,
            queue: Arc::new(Mutex::new(VecDeque::new())),
        };
        rate_limiter.recv(tasks_rx);
        rate_limiter
    }
}

#[derive(Clone)]
pub(crate) struct RateLimiter {
    buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
    queue: Arc<Mutex<VecDeque<TaskMessage>>>,
}

impl RateLimiter {
    pub fn task<S>(&self, builder: RequestBuilder<S>) -> TaskBuilder<S>
    where
        S: KrakenSigner + Unpin,
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
                let is_first_task = {
                    let mut guard = queue.lock().await;

                    // high priority at the beginning
                    guard.push_back(task_message);
                    guard
                        .make_contiguous()
                        .sort_by(|a, b| b.priority.cmp(&a.priority));
                    guard.len() == 1
                };

                if is_first_task {
                    Self::handler(buckets.clone(), queue.clone()).await;
                }
            }
        });
    }

    async fn handler<'a>(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        queue: Arc<Mutex<VecDeque<TaskMessage>>>,
    ) {
        let buckets = buckets.clone();
        let queue = queue.clone();
        actix_rt::spawn(async move {
            loop {
                let TaskMessage {
                    priority,
                    costs,
                    tx,
                } = match queue.lock().await.pop_front() {
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

    async fn timeout<'a>(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &'a TaskCosts,
    ) -> KrakenResult<Option<Duration>> {
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

    async fn set_costs<'a>(
        buckets: Arc<HashMap<BucketName, Mutex<RateLimiterBucket>>>,
        costs: &'a TaskCosts,
    ) -> KrakenResult<()> {
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
    mode: RateLimiterBucketMode,
    time_instant: Instant,
    delay: Instant,
    interval: Duration,
    limit: u32,
    amount: u32,
}

impl Default for RateLimiterBucket {
    fn default() -> Self {
        Self {
            mode: RateLimiterBucketMode::default(),
            time_instant: Instant::now(),
            delay: Instant::now(),
            interval: Duration::default(),
            limit: 0,
            amount: 0,
        }
    }
}

impl RateLimiterBucket {
    pub fn mode(mut self, mode: RateLimiterBucketMode) -> Self {
        self.mode = mode;
        self
    }

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
        match self.mode {
            RateLimiterBucketMode::Interval => {
                let elapsed = Instant::now().duration_since(self.time_instant);
                if elapsed > self.interval {
                    self.time_instant = Instant::now();
                    self.amount = 0;
                }
            }
            RateLimiterBucketMode::KrakenDecrease => {
                let elapsed = Instant::now().duration_since(self.time_instant);
                let available =
                    (elapsed.as_secs_f32() / self.interval.as_secs_f32()).floor() as u32;
                if available > 0 {
                    self.time_instant = Instant::now();
                    self.amount = if self.amount > available {
                        self.amount - available
                    } else {
                        0
                    };
                }
            }
        }
    }

    fn get_timeout(&self) -> Duration {
        match self.mode {
            RateLimiterBucketMode::Interval => {
                let elapsed = Instant::now().duration_since(self.time_instant);
                self.interval - elapsed
            }
            RateLimiterBucketMode::KrakenDecrease => self.interval,
        }
    }
}

pub(crate) enum RateLimiterBucketMode {
    Interval,
    KrakenDecrease,
}

impl Default for RateLimiterBucketMode {
    fn default() -> Self {
        RateLimiterBucketMode::Interval
    }
}

pub(crate) struct TaskBuilder<S>
where
    S: KrakenSigner + Unpin + 'static,
{
    priority: u8,
    costs: TaskCosts,
    req_builder: RequestBuilder<S>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
}

impl<S> TaskBuilder<S>
where
    S: KrakenSigner + Unpin + 'static,
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

#[derive(Debug)]
pub struct TaskMetadata {
    pub costs: TaskCosts,
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU8, Ordering};

    use super::*;

    use crate::api::spot::AssetInfoResponse;
    use crate::client::RateLimiterTier;
    use crate::{ApiCred, Proxy, SpotApi};

    pub static CCX_KRAKEN_API_PREFIX: &str = "CCX_KRAKEN_API";

    #[actix_rt::test]
    async fn test_rate_limiter_queue() {
        let signer = ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let tier = RateLimiterTier::Starter;
        let spot_api = SpotApi::new(signer, proxy, tier);

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
                .cost("interval_1__limit_1", 1)
                .cost("interval_10__limit_2", 1)
                .send::<AssetInfoResponse>()
                .await;

            assert!(task_res.is_ok());
        }

        assert!(instant.elapsed() >= Duration::from_secs(30));
    }

    #[actix_rt::test]
    async fn test_rate_limiter_metadata() {
        let signer = ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let tier = RateLimiterTier::Starter;
        let spot_api = SpotApi::new(signer, proxy, tier);

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
                .cost("interval_1__limit_1", 1)
                .cost("interval_10__limit_2", 1)
                .send::<AssetInfoResponse>();

            assert_eq!(task.metadata().costs.get("interval_1__limit_1"), Some(&1));
            assert_eq!(task.metadata().costs.get("interval_10__limit_2"), Some(&1));
        }
    }

    #[actix_rt::test]
    async fn test_rate_limiter_delay() {
        let signer = ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let tier = RateLimiterTier::Starter;
        let spot_api = SpotApi::new(signer, proxy, tier);

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "delay_10__interval_1__limit_1",
                RateLimiterBucket::default()
                    .delay(Duration::from_secs(3))
                    .interval(Duration::from_secs(3))
                    .limit(1),
            )
            .start();

        let instant = Instant::now();
        for _i in 1..=2 {
            let task_res = rate_limiter
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
                .cost("delay_10__interval_1__limit_1", 1)
                .send::<AssetInfoResponse>()
                .await;

            assert!(task_res.is_ok());
        }

        assert!(instant.elapsed() >= Duration::from_secs(6));
    }

    #[actix_rt::test]
    async fn test_rate_limiter_wrong_bucket() {
        let signer = ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let tier = RateLimiterTier::Starter;
        let spot_api = SpotApi::new(signer, proxy, tier);

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
            .cost("interval_1__limit_1", 1)
            .send::<AssetInfoResponse>()
            .await;
        assert!(task_res.is_err())
    }

    #[actix_rt::test]
    async fn test_rate_limiter_decrease() {
        let signer = ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let tier = RateLimiterTier::Starter;
        let spot_api = SpotApi::new(signer, proxy, tier);

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "interval_3__limit_5",
                RateLimiterBucket::default()
                    .mode(RateLimiterBucketMode::KrakenDecrease)
                    .interval(Duration::from_secs(3))
                    .limit(5),
            )
            .start();

        let instant = Instant::now();
        for _i in 1..10 {
            let _task_res = rate_limiter
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
                .cost("interval_3__limit_5", 1)
                .send::<AssetInfoResponse>()
                .await;
        }

        assert!(instant.elapsed() >= Duration::from_secs(13));
    }

    #[actix_rt::test]
    async fn test_rate_limiter_priority() {
        let signer = ApiCred::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let proxy = Proxy::from_env_with_prefix(CCX_KRAKEN_API_PREFIX);
        let tier = RateLimiterTier::Starter;
        let spot_api = SpotApi::new(signer, proxy, tier);

        let rate_limiter = RateLimiterBuilder::default()
            .bucket(
                "interval_3__limit_5",
                RateLimiterBucket::default()
                    .mode(RateLimiterBucketMode::KrakenDecrease)
                    .interval(Duration::from_secs(3))
                    .limit(5),
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
                    .cost("interval_3__limit_5", 1)
                    .priority(1)
                    .send::<AssetInfoResponse>()
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
                    .cost("interval_3__limit_5", 1)
                    .send::<AssetInfoResponse>()
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
        assert!(instant.elapsed() >= Duration::from_secs(13));
    }
}
