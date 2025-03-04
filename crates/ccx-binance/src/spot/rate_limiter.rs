use std::time::Duration;

use super::types::rate_limits::RateLimitType;

pub use ccx_lib::rate_limiter::RateLimiterBucket;
pub use ccx_lib::rate_limiter::RateLimiterError;

const SECOND: u64 = 1;
const MINUTE: u64 = 60 * SECOND;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

fn buckets<B: FromIterator<RateLimiterBucket>>(list: impl IntoIterator<Item = (u64, u32)>) -> B {
    list.into_iter()
        .map(|(interval, limit)| RateLimiterBucket::new_now(Duration::from_secs(interval), limit))
        .collect()
}

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct RateLimiter(ccx_lib::rate_limiter::RateLimiter<RateLimitType>);

impl RateLimiter {
    pub fn spawn() -> Self {
        Self(ccx_lib::rate_limiter::RateLimiter::<RateLimitType>::spawn(
            |rate_limit_type| match rate_limit_type {
                RateLimitType::RequestWeight => buckets([(1 * MINUTE, 6_000)]),
                RateLimitType::Orders => buckets([(10 * SECOND, 100), (1 * DAY, 200_000)]),
                RateLimitType::RawRequests => buckets([(5 * MINUTE, 61_000)]),
            },
        ))
    }
}
