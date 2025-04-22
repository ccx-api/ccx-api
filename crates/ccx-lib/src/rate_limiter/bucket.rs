use std::time::Duration;

use tokio::time::Instant;

pub trait RateLimiterBucket: std::fmt::Debug + Send + Sync {
    fn reset_if_expired(&mut self, now: Instant) -> bool;

    fn is_empty(&self) -> bool;

    fn get_timeout(&self, now: Instant) -> Duration;

    fn can_increase_cost(&self, cost: u32) -> bool;

    fn increase(&mut self, cost: u32) -> bool;
}

/// Rate limiter based on Fixed Window Rate limiter
#[derive(Debug)]
pub struct RateLimiterBucketWindow {
    /// Time interval for the limit to be applied.
    interval: Duration,
    /// The limit of the bucket.
    limit: u32,
    /// The time when the current interval started.
    started_at: Instant,
    /// The amount of requests made in the current interval.
    amount: u32,
}

impl RateLimiterBucketWindow {
    pub fn new_now(interval: Duration, limit: u32) -> Self {
        Self {
            interval,
            limit,
            started_at: Instant::now(),
            amount: 0,
        }
    }
}

impl RateLimiterBucket for RateLimiterBucketWindow {
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

    fn can_increase_cost(&self, cost: u32) -> bool {
        self.amount + cost <= self.limit
    }

    fn increase(&mut self, cost: u32) -> bool {
        if self.can_increase_cost(cost) {
            self.amount += cost;
            true
        } else {
            false
        }
    }
}

/// Rate limiter based on Token Bucket rate limiter
#[derive(Debug)]
pub struct RateLimiterBucketToken {
    /// Time interval for the limit to be applied.
    interval: Duration,
    /// The limit of the bucket.
    limit: u32,
    /// The time when the current interval started.
    last_check: Instant,
    /// The amount of requests made in the current interval.
    count: u32,
}

impl RateLimiterBucketToken {
    pub fn new_now(interval: Duration, limit: u32) -> Self {
        Self {
            interval,
            limit,
            last_check: Instant::now(),
            count: 0,
        }
    }
}

impl RateLimiterBucket for RateLimiterBucketToken {
    fn reset_if_expired(&mut self, now: Instant) -> bool {
        let elapsed = now.saturating_duration_since(self.last_check);
        let decrease = (elapsed.as_secs_f32() / self.interval.as_secs_f32()).floor() as u32;
        if decrease > 0 {
            self.last_check = now;
            self.count = if self.count > decrease {
                self.count - decrease
            } else {
                0
            };
        }

        tracing::trace!(self.count, "new count");

        decrease > 0
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn get_timeout(&self, _now: Instant) -> Duration {
        self.interval
    }

    fn can_increase_cost(&self, cost: u32) -> bool {
        self.count + cost < self.limit
    }

    fn increase(&mut self, cost: u32) -> bool {
        if self.can_increase_cost(cost) {
            self.count += cost;
            true
        } else {
            false
        }
    }
}

impl From<RateLimiterBucketWindow> for Box<dyn RateLimiterBucket> {
    fn from(value: RateLimiterBucketWindow) -> Self {
        Box::new(value)
    }
}

impl From<RateLimiterBucketToken> for Box<dyn RateLimiterBucket> {
    fn from(value: RateLimiterBucketToken) -> Self {
        Box::new(value)
    }
}
