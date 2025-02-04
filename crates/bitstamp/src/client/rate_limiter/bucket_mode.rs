#[derive(Default)]
pub(crate) enum RateLimiterBucketMode {
    #[default]
    Interval,
    // BitstampDecrease,
}
