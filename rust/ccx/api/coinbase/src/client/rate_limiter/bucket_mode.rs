pub(crate) enum RateLimiterBucketMode {
    Interval,
    // CoinbaseDecrease,
}

impl Default for RateLimiterBucketMode {
    fn default() -> Self {
        RateLimiterBucketMode::Interval
    }
}
