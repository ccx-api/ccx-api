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

pub(crate) type RateLimitKey = (&'static RateLimitType, &'static str);

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct RateLimiter(ccx_lib::rate_limiter::RateLimiter<RateLimitKey>);

impl RateLimiter {
    pub fn spawn() -> Self {
        Self(ccx_lib::rate_limiter::RateLimiter::<RateLimitKey>::spawn(
            // based on https://www.gate.io/docs/developers/apiv4/en/#frequency-limit-rule
            |(rate_limit_type, _endpoint)| match rate_limit_type {
                RateLimitType::Public => buckets([(Duration::from_secs(10), 200)]),
                RateLimitType::WalletWithdraw => buckets([(Duration::from_secs(3), 1)]),
                RateLimitType::WalletTransferOrBalance => buckets([(Duration::from_secs(10), 80)]),
                RateLimitType::WalletOther => buckets([(Duration::from_secs(10), 200)]),
                RateLimitType::SpotOrderCreateChange => buckets([(Duration::from_secs(1), 10)]),
                RateLimitType::SpotOrderCancel => buckets([(Duration::from_secs(1), 200)]),
                RateLimitType::SpotOther => buckets([(Duration::from_secs(10), 200)]),
                RateLimitType::Other => buckets([(Duration::from_secs(10), 150)]),
            },
        ))
    }
}
