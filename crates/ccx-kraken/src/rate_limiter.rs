use std::time::Duration;

use ccx_lib::rate_limiter::{RateLimiterBucket, RateLimiterBucketToken, RateLimiterBucketWindow};

use crate::types::rate_limits::RateLimitType;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum RateLimitKey {
    Public,
    Private,
}

impl From<&RateLimitType> for RateLimitKey {
    fn from(value: &RateLimitType) -> Self {
        match value {
            RateLimitType::Public => Self::Public,
            RateLimitType::Private(_) => Self::Private,
        }
    }
}

#[derive(Default)]
pub enum Tier {
    #[default]
    Starter,
    Intermediate,
    Pro,
}

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct RateLimiter(ccx_lib::rate_limiter::RateLimiter<RateLimitKey>);

impl RateLimiter {
    pub fn spawn(tier: Tier) -> Self {
        Self(ccx_lib::rate_limiter::RateLimiter::<RateLimitKey>::spawn(
            move |ty| -> Vec<Box<dyn RateLimiterBucket>> {
                match ty {
                    // for the public API it's not very clear what's the actual rate limits
                    RateLimitKey::Public => vec![Box::new(RateLimiterBucketWindow::new_now(
                        Duration::from_secs(1),
                        100,
                    ))],
                    RateLimitKey::Private => match tier {
                        // https://docs.kraken.com/api/docs/guides/spot-rest-ratelimits/
                        Tier::Starter => vec![Box::new(RateLimiterBucketToken::new_now(
                            Duration::from_secs(3),
                            15,
                        ))],
                        Tier::Intermediate => vec![Box::new(RateLimiterBucketToken::new_now(
                            Duration::from_secs(2),
                            20,
                        ))],
                        Tier::Pro => vec![Box::new(RateLimiterBucketToken::new_now(
                            Duration::from_secs(1),
                            20,
                        ))],
                    },
                }
            },
        ))
    }
}
