use std::time::{Duration, Instant};

use crate::client::RateLimiterBucketMode;

pub(crate) struct RateLimiterBucket {
    pub(super) mode: RateLimiterBucketMode,
    pub(super) time_instant: Instant,
    pub(super) delay: Instant,
    pub(super) interval: Duration,
    pub(super) limit: u32,
    pub(super) amount: u32,
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

    pub(super) fn update_state(&mut self) {
        match self.mode {
            RateLimiterBucketMode::Interval => {
                let elapsed = Instant::now().duration_since(self.time_instant);
                if elapsed > self.interval {
                    self.time_instant = Instant::now();
                    self.amount = 0;
                }
            } // RateLimiterBucketMode::CoinbaseDecrease => {
              //     let elapsed = Instant::now().duration_since(self.time_instant);
              //     let available =
              //         (elapsed.as_secs_f32() / self.interval.as_secs_f32()).floor() as u32;
              //     if available > 0 {
              //         self.time_instant = Instant::now();
              //         self.amount = if self.amount > available {
              //             self.amount - available
              //         } else {
              //             0
              //         };
              //     }
              // }
        }
    }

    pub(super) fn get_timeout(&self) -> Duration {
        match self.mode {
            RateLimiterBucketMode::Interval => {
                let elapsed = Instant::now().duration_since(self.time_instant);
                self.interval - elapsed
            } // RateLimiterBucketMode::CoinbaseDecrease => self.interval,
        }
    }
}
