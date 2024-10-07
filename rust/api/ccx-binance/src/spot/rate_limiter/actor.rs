use std::collections::HashMap;
use std::time::Duration;

use futures::channel::mpsc;
use futures::StreamExt;
use strum::IntoEnumIterator;
use tokio::select;
use tokio::time::interval;
use tokio::time::Instant;
use tokio::time::MissedTickBehavior;

use crate::spot::rate_limiter::queue::Queue;
use crate::spot::rate_limiter::types::RateLimiterMessage;
use crate::spot::rate_limiter::types::TaskCosts;
use crate::spot::types::rate_limits::RateLimitType;

pub struct RateLimiterActor {
    buckets: HashMap<RateLimitType, Vec<RateLimiterBucket>>,
    queue: Queue,
}

pub(crate) struct RateLimiterBucket {
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
    fn new_now(interval_seconds: u64, limit: u32) -> Self {
        let interval = Duration::from_secs(interval_seconds);
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

impl RateLimiterActor {
    pub fn new_clean() -> Self {
        let queue = Queue::new();
        let buckets = RateLimitType::iter().map(|typ| (typ, Vec::new())).collect();

        Self { buckets, queue }
    }

    pub fn new_prepared() -> Self {
        const SECOND: u64 = 1;
        const MINUTE: u64 = 60 * SECOND;
        const HOUR: u64 = 60 * MINUTE;
        const DAY: u64 = 24 * HOUR;

        fn buckets<B: FromIterator<RateLimiterBucket>>(
            list: impl IntoIterator<Item = (u64, u32)>,
        ) -> B {
            list.into_iter()
                .map(|(interval, limit)| RateLimiterBucket::new_now(interval, limit))
                .collect()
        }

        let mut this = Self::new_clean();

        this.buckets.extend([
            (RateLimitType::RequestWeight, buckets([(1 * MINUTE, 6_000)])),
            (
                RateLimitType::Orders,
                buckets([(10 * SECOND, 100), (1 * DAY, 200_000)]),
            ),
            (RateLimitType::RawRequests, buckets([(5 * MINUTE, 61_000)])),
        ]);

        this
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<RateLimiterMessage>) {
        enum Event {
            TaskMessage(Option<RateLimiterMessage>),
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
                            // tracing::debug!("RateLimiter: enqueue task");
                            let limit_reached = self.check_limits(task.costs, current_now);

                            if let Some(_delay) = limit_reached {
                                // Если лимит превышен, то добавляем сообщение в очередь.
                                self.queue.add(task);
                            } else {
                                // Если лимит не превышен, то обрабатываем сообщение.
                                self.handle_costs(task.costs);
                                let _ = task.tx.send(());
                            }
                        }
                    }
                }
                Event::TaskMessage(None) => {
                    // Очередь сообщений закрыта (все отправители удалены), завершаем обработку.
                    // tracing::debug!("RateLimiter: stop queue handler (all senders are dropped)");
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

    /// Возвращает время, через которое можно будет обработать следующее сообщение, или `None`,
    /// если обработка сообщения возможна сразу.
    fn check_limits(&self, costs: TaskCosts, now: Instant) -> Option<Duration> {
        // Проверяем, не превышен ли лимит в каком-либо из бакетов.
        let mut limit_reached = None;
        for (typ, cost) in costs {
            // Safety: `buckets` is initialized in `new` with all possible
            // `RateLimitType` values.
            let buckets = self.buckets.get(typ).unwrap();
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

    fn handle_costs(&mut self, costs: TaskCosts) {
        for (typ, cost) in costs {
            // Safety: `buckets` is initialized in `new` with all possible `RateLimitType` values.
            let buckets = self.buckets.get_mut(typ).unwrap();
            for bucket in buckets {
                bucket.amount += cost;
            }
        }
    }
}
