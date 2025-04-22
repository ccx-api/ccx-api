use std::time::Duration;

use super::types::rate_limits::RateLimitType;

pub use ccx_lib::rate_limiter::RateLimiterBucketWindow;
pub use ccx_lib::rate_limiter::RateLimiterError;

const SECOND: u64 = 1;

fn buckets<B: FromIterator<RateLimiterBucketWindow>>(
    list: impl IntoIterator<Item = (u64, u32)>,
) -> B {
    list.into_iter()
        .map(|(interval, limit)| {
            RateLimiterBucketWindow::new_now(Duration::from_secs(interval), limit)
        })
        .collect()
}

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct RateLimiter(ccx_lib::rate_limiter::RateLimiter<RateLimitType>);

impl RateLimiter {
    pub fn spawn() -> Self {
        Self(ccx_lib::rate_limiter::RateLimiter::<RateLimitType>::spawn(
            // based on [docs](https://mexcdevelop.github.io/apidocs/spot_v3_en/#limits)
            |_endpoint| buckets([(10 * SECOND, 500)]),
        ))
    }
}
