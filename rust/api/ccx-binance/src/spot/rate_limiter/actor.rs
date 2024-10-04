use std::borrow::Cow;
use std::collections::HashMap;
use std::pin::pin;
use std::pin::Pin;
use std::time::Duration;
use std::time::Instant;

use futures::channel::mpsc;
use futures::StreamExt;
use tokio::select;
use tokio::time::sleep;
use tokio::time::Sleep;

use crate::spot::rate_limiter::queue::Queue;
use crate::spot::rate_limiter::types::TaskCosts;
use crate::spot::rate_limiter::types::TaskMessage;
use crate::spot::types::rate_limits::RateLimitType;

type BucketName = Cow<'static, str>;

struct RateLimiterActor {
    buckets: HashMap<RateLimitType, Vec<RateLimiterBucket>>,
    queue: Queue,
    timeout: Option<(Pin<Sleep>, Duration)>,
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

    fn reset_if_expired(&mut self) {
        self.reset_if_expired_(Instant::now());
    }

    fn reset_if_expired_(&mut self, now: Instant) {
        let elapsed = now.saturating_duration_since(self.started_at);
        if elapsed > self.interval {
            self.started_at = now;
            self.amount = 0;
        }
    }

    fn get_timeout(&self) -> Duration {
        self.get_timeout_(Instant::now())
    }

    fn get_timeout_(&self, now: Instant) -> Duration {
        let elapsed = now.saturating_duration_since(self.started_at);
        self.interval - elapsed
    }
}

impl RateLimiterActor {
    pub fn new() -> Self {
        let queue = Queue::new();
        let timeout = None;

        let buckets = [
            (
                RateLimitType::RequestWeight,
                vec![RateLimiterBucket::new_now(60, 6000)],
            ),
            (
                RateLimitType::Orders,
                vec![
                    RateLimiterBucket::new_now(10, 100),
                    RateLimiterBucket::new_now(60 * 60 * 24, 200_000),
                ],
            ),
            (
                RateLimitType::RawRequests,
                vec![RateLimiterBucket::new_now(5 * 60, 61_000)],
            ),
        ]
        .into_iter()
        .collect();

        Self {
            buckets,
            queue,
            timeout,
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<TaskMessage>) {
        enum Event {
            TaskMessage(Option<TaskMessage>),
            Timeout,
        }

        loop {
            let event = match &mut self.timeout {
                None => Event::TaskMessage(rx.next().await),
                Some((fut, timeout)) => {
                    let event = {
                        select! {
                            message = rx.next() => Event::TaskMessage(message),
                            () = fut => Event::Timeout,
                        }
                    };
                    event
                }
            };

            match event {
                Event::TaskMessage(Some(message)) => {
                    match message {
                        TaskMessage::Enqueue(task) => {
                            // tracing::debug!("RateLimiter: enqueue task");
                            // Работаем от единой точки времени, чтобы избежать проблем с
                            // синхронизацией.
                            let now = Instant::now();

                            // Предварительно очищаем бакеты, период которых завершился.
                            self.reset_expired_buckets_(now);

                            let limit_reached = self.check_limits(task.costs, now);

                            if let Some(new_timeout) = limit_reached {
                                // Если лимит превышен, то добавляем сообщение в очередь и
                                // обновляем таймаут, если он должен сократиться.
                                self.queue.add(task);
                                self.handle_timeout(new_timeout);
                            } else {
                                // Если лимит не превышен, то обрабатываем сообщение.
                                self.handle_costs(task.costs);
                                let _ = task.tx.send(());
                            }
                        }
                    }
                }
                Event::TaskMessage(None) => {
                    // tracing::debug!("RateLimiter: stop queue handler (all senders are dropped)");
                    break;
                }
                Event::Timeout => {
                    // tracing::debug!("RateLimiter: timeout");

                    // Сбрасываем таймаут, так как он уже прошел.
                    let _ = self.timeout.take();

                    let now = Instant::now();
                    self.reset_expired_buckets_(now);

                    // Забираем из очереди задачи, время которых пришло.
                    loop {
                        let Some(task) = self.queue.first() else {
                            // Очередь пуста, завершаем обработку.
                            debug_assert!(self.timeout.is_none());
                            break;
                        };

                        let limit_reached = self.check_limits(task.costs, now);

                        if let Some(new_timeout) = limit_reached {
                            // Если лимит превышен, то оставляем очередь в покое и
                            // обновляем таймаут, если он должен сократиться.
                            self.handle_timeout(new_timeout);
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
        }
    }

    fn reset_expired_buckets(&mut self) {
        self.reset_expired_buckets_(Instant::now());
    }

    fn reset_expired_buckets_(&mut self, now: Instant) {
        for buckets in self.buckets.values_mut() {
            for bucket in buckets {
                bucket.reset_if_expired_(now);
            }
        }
    }

    fn check_limits(&self, costs: TaskCosts, now: Instant) -> Option<Duration> {
        // Проверяем, не превышен ли лимит в каком-либо из бакетов.
        let mut limit_reached = None;
        for (typ, cost) in costs {
            // Safety: `buckets` is initialized in `new` with all possible
            // `RateLimitType` values.
            let buckets = self.buckets.get(typ).unwrap();
            for bucket in buckets {
                if bucket.amount + cost > bucket.limit {
                    let new_timeout = bucket.get_timeout_(now);
                    limit_reached = limit_reached
                        .map(|old| old.min(new_timeout))
                        .or(Some(new_timeout));
                }
            }
        }
    }

    fn handle_timeout(&mut self, new_timeout: Duration) {
        match &self.timeout {
            Some((_fut, old)) if *old <= new_timeout => {}
            _ => self.timeout = Some((sleep(new_timeout), new_timeout)),
        };
    }

    fn handle_costs(&mut self, costs: TaskCosts) {
        for (typ, cost) in costs {
            // Safety: `buckets` is initialized in `new` with all possible
            // `RateLimitType` values.
            let buckets = self.buckets.get_mut(typ).unwrap();
            for bucket in buckets {
                bucket.amount += cost;
            }
        }
    }
}
