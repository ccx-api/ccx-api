use std::time::Duration;

use super::types::rate_limits::RateLimitType;

pub use ccx_lib::rate_limiter::RateLimiterBucket;
pub use ccx_lib::rate_limiter::RateLimiterError;

fn buckets<B: FromIterator<RateLimiterBucket>>(
    list: impl IntoIterator<Item = (Duration, u32)>,
) -> B {
    list.into_iter()
        .map(|(interval, limit)| RateLimiterBucket::new_now(interval, limit))
        .collect()
}

pub(crate) type RateLimitKey = ();

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct RateLimiter(ccx_lib::rate_limiter::RateLimiter<RateLimitKey>);

// TODO: refactor this big
impl RateLimiter {
    pub fn spawn() -> Self {
        Self(ccx_lib::rate_limiter::RateLimiter::<RateLimitKey>::spawn(
            |_| buckets([(Duration::from_secs(5), 15)]),
        ))
    }
}
