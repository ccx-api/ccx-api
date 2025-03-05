use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

use futures::StreamExt;
use futures::channel::mpsc;
use tokio::select;
use tokio::time::Instant;
use tokio::time::MissedTickBehavior;
use tokio::time::interval;

use crate::rate_limiter::queue::Queue;
use crate::rate_limiter::types::RateLimiterMessage;
use crate::rate_limiter::types::TaskCosts;

pub struct RateLimiterActor<RateLimitType, BucketInit>
where
    RateLimitType: 'static,
{
    buckets: HashMap<&'static RateLimitType, Vec<RateLimiterBucket>>,
    bucket_init: BucketInit,
    queue: Queue<RateLimitType>,
}

#[derive(Debug)]
pub struct RateLimiterBucket {
    /// Time interval for the limit to be applied.
    interval: Duration,
    /// The limit of the bucket.
    limit: u32,
    /// The time when the current interval started.
    started_at: Instant,
    /// The amount of requests made in the current interval.
    amount: u32,
}

impl RateLimiterBucket {
    pub fn new_now(interval: Duration, limit: u32) -> Self {
        Self {
            interval,
            limit,
            started_at: Instant::now(),
            amount: 0,
        }
    }

    fn reset_if_expired(&mut self, now: Instant) -> bool {
        let elapsed = now.saturating_duration_since(self.started_at);
        let is_expired = self.interval < elapsed;
        if is_expired {
            self.amount = 0;
        }
        // Сдвигаем окно, если бакет пустой.
        if self.is_empty() {
            self.started_at = now;
        }
        is_expired
    }

    fn is_empty(&self) -> bool {
        self.amount == 0
    }

    fn get_timeout(&self, now: Instant) -> Duration {
        let elapsed = now.saturating_duration_since(self.started_at);
        self.interval - elapsed
    }
}

impl<RateLimitType, BucketInit> RateLimiterActor<RateLimitType, BucketInit>
where
    RateLimitType: Eq + Hash + std::fmt::Debug,
    BucketInit: Fn(&RateLimitType) -> Vec<RateLimiterBucket>,
{
    pub fn with_bucket_initializer(bucket_init: BucketInit) -> Self {
        Self {
            buckets: Default::default(),
            queue: Queue::new(),
            bucket_init,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn run(mut self, mut rx: mpsc::Receiver<RateLimiterMessage<RateLimitType>>) {
        enum Event<RL: 'static> {
            TaskMessage(Option<RateLimiterMessage<RL>>),
            CheckExpired(Instant),
        }

        let mut interval = interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        // Текущее время, чтобы избежать проблем с синхронизацией бакетов.
        let mut current_now = Instant::now();

        loop {
            let event = select! {
                message = rx.next() => Event::TaskMessage(message),
                now = interval.tick() => Event::CheckExpired(now),
            };

            match event {
                Event::CheckExpired(now) => {
                    tracing::trace!("check expired");
                    current_now = now;

                    let has_expired = self.reset_expired_buckets(now);

                    if has_expired && !self.queue.is_empty() {
                        // Забираем из очереди задачи, время которых пришло.
                        loop {
                            let Some(task) = self.queue.first() else {
                                // Очередь пуста, завершаем обработку.
                                break;
                            };

                            let limit_reached = self.check_limits(task.costs, now);

                            if let Some(_delay) = limit_reached {
                                // Если лимит превышен, то оставляем очередь в покое.
                                // Это может измениться до истечения таймаута, если в очередь
                                // добавятся новые задачи.
                                break;
                            } else {
                                // Если лимит не превышен, то обрабатываем сообщение.
                                self.handle_costs(task.costs);
                                // Safety: task points to the first element of the queue,
                                //  so pop is guaranteed to return Some.
                                let task = self.queue.pop().unwrap();
                                let _ = task.tx.send(());
                            }
                        }
                    }
                }
                Event::TaskMessage(Some(message)) => {
                    match message {
                        RateLimiterMessage::Enqueue(task) => {
                            tracing::trace!(?task, "new task");
                            self.check_or_insert_bucket(task.costs);

                            // tracing::debug!("RateLimiter: enqueue task");
                            let limit_reached = self.check_limits(task.costs, current_now);

                            if let Some(delay) = limit_reached {
                                tracing::debug!(?delay, ?task, "Delay task for later execution");
                                // Если лимит превышен, то добавляем сообщение в очередь.
                                self.queue.add(task);
                            } else {
                                tracing::trace!(?task, "Execute task immediately");
                                // Если лимит не превышен, то обрабатываем сообщение.
                                self.handle_costs(task.costs);
                                let _ = task.tx.send(());
                            }
                        }
                    }
                }
                Event::TaskMessage(None) => {
                    // Очередь сообщений закрыта (все отправители удалены), завершаем обработку.
                    tracing::debug!("RateLimiter: stop queue handler (all senders are dropped)");
                    break;
                }
            }
        }
    }

    fn reset_expired_buckets(&mut self, now: Instant) -> bool {
        let mut has_expired = false;
        for buckets in self.buckets.values_mut() {
            for bucket in buckets {
                has_expired |= bucket.reset_if_expired(now);
            }
        }
        has_expired
    }

    fn check_or_insert_bucket(&mut self, costs: TaskCosts<RateLimitType>) {
        for (typ, _cost) in costs {
            self.buckets.entry(typ).or_insert_with(|| {
                let bucket = (self.bucket_init)(typ);
                tracing::trace!(?typ, ?bucket, "Create new bucket");

                bucket
            });
        }
    }

    /// Возвращает время, через которое можно будет обработать следующее сообщение, или `None`,
    /// если обработка сообщения возможна сразу.
    fn check_limits(&self, costs: TaskCosts<RateLimitType>, now: Instant) -> Option<Duration> {
        // Проверяем, не превышен ли лимит в каком-либо из бакетов.
        let mut limit_reached = None;
        for (typ, cost) in costs {
            // Safety: `buckets` is initialized in `new` with all possible
            // `RateLimitType` values.
            let Some(buckets) = self.buckets.get(typ) else {
                tracing::warn!(?typ, "Specified bucket is not found");

                return None;
            };
            for bucket in buckets {
                if bucket.amount + *cost > bucket.limit {
                    let new_timeout = bucket.get_timeout(now);
                    limit_reached = limit_reached
                        .map(|old| new_timeout.max(old))
                        .or(Some(new_timeout));
                }
            }
        }
        limit_reached
    }

    fn handle_costs(&mut self, costs: TaskCosts<RateLimitType>) {
        for (typ, cost) in costs {
            let Some(buckets) = self.buckets.get_mut(typ) else {
                tracing::warn!(?typ, "Specified bucket is not found");

                return;
            };
            for bucket in buckets {
                bucket.amount += cost;
            }
        }
    }
}
