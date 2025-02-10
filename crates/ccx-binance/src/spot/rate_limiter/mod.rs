mod actor;
mod error;
mod handle;
mod queue;
mod types;

pub use error::RateLimiterError;
pub use handle::RateLimiter;

// pub const RL_WEIGHT_PER_MINUTE: &str = "weight_per_minute";
// pub const RL_ORDERS_PER_SECOND: &str = "orders_per_second";
// pub const RL_ORDERS_PER_DAY: &str = "orders_per_day";

// Actual rate limits for Binance Spot API:
//
// RateLimit {
//     rate_limit_type: RequestWeight,
//     interval: Minute,
//     interval_num: 1,
//     limit: 6000,
// }
// RateLimit {
//     rate_limit_type: Orders,
//     interval: Second,
//     interval_num: 10,
//     limit: 100,
// }
// RateLimit {
//     rate_limit_type: Orders,
//     interval: Day,
//     interval_num: 1,
//     limit: 200000,
// }
// RateLimit {
//     rate_limit_type: RawRequests,
//     interval: Minute,
//     interval_num: 5,
//     limit: 61000,
// }
