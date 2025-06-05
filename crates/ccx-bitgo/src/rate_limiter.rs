use std::time::Duration;

use ccx_lib::rate_limiter::{RateLimiterBucket, RateLimiterBucketWindow};

use crate::types::rate_limits::RateLimitType;

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct RateLimiter(ccx_lib::rate_limiter::RateLimiter<RateLimitType>);

impl RateLimiter {
    pub fn spawn() -> Self {
        Self(ccx_lib::rate_limiter::RateLimiter::<RateLimitType>::spawn(
            move |ty| -> Vec<Box<dyn RateLimiterBucket>> {
                // based on https://developers.bitgo.com/api/overview
                match ty {
                    RateLimitType::Public => vec![Box::new(RateLimiterBucketWindow::new_now(
                        Duration::from_secs(60),
                        1200,
                    ))],
                    RateLimitType::Authenticated => vec![Box::new(
                        RateLimiterBucketWindow::new_now(Duration::from_secs(60), 360),
                    )],
                }
            },
        ))
    }
}
