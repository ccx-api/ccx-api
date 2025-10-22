mod bucket;
mod bucket_mode;
mod limiter;
mod limiter_builder;
mod queue;
mod task;
mod task_builder;
mod task_message;
mod task_metadata;

use std::borrow::Cow;

#[allow(unused_imports)]
pub use bucket::*;
#[allow(unused_imports)]
pub use bucket_mode::*;
#[allow(unused_imports)]
pub use limiter::*;
#[allow(unused_imports)]
pub use limiter_builder::*;
use queue::*;
#[allow(unused_imports)]
pub use task::*;
use task_builder::*;
use task_message::*;
#[allow(unused_imports)]
pub use task_metadata::*;

type BucketName = Cow<'static, str>;

// #[cfg(test)]
// mod tests {
//     use std::sync::atomic::{AtomicU8, Ordering};
//
//     use super::*;
//
//     use crate::api::spot::AssetInfoResponse;
//     use crate::client::RateLimiterTier;
//     use crate::{ApiCred, Proxy, SpotApi};
//
//     pub static CCX_COINBASE_API_PREFIX: &str = "CCX_COINBASE_API";
//
//     #[actix_rt::test]
//     async fn test_rate_limiter_queue() {
//         let signer = ApiCred::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let tier = RateLimiterTier::Starter;
//         let spot_api = SpotApi::new(signer, proxy, tier);
//
//         let rate_limiter = RateLimiterBuilder::default()
//             .bucket(
//                 "interval_1__limit_1",
//                 RateLimiterBucket::default()
//                     .interval(Duration::from_secs(1))
//                     .limit(1),
//             )
//             .bucket(
//                 "interval_10__limit_2",
//                 RateLimiterBucket::default()
//                     .interval(Duration::from_secs(10))
//                     .limit(2),
//             )
//             .start();
//
//         let instant = Instant::now();
//         for _i in 1..=8 {
//             let task_res = rate_limiter
//                 .task(
//                     spot_api
//                         .client
//                         .get("/0/public/Assets")
//                         .unwrap()
//                         .try_query_arg("pairs", &None::<&str>)
//                         .unwrap()
//                         .try_query_arg("info", &None::<&str>)
//                         .unwrap(),
//                 )
//                 .cost("interval_1__limit_1", 1)
//                 .cost("interval_10__limit_2", 1)
//                 .send::<AssetInfoResponse>()
//                 .await;
//
//             assert!(task_res.is_ok());
//         }
//
//         assert!(instant.elapsed() >= Duration::from_secs(30));
//     }
//
//     #[actix_rt::test]
//     async fn test_rate_limiter_metadata() {
//         let signer = ApiCred::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let tier = RateLimiterTier::Starter;
//         let spot_api = SpotApi::new(signer, proxy, tier);
//
//         let rate_limiter = RateLimiterBuilder::default()
//             .bucket(
//                 "interval_1__limit_1",
//                 RateLimiterBucket::default()
//                     .interval(Duration::from_secs(1))
//                     .limit(1),
//             )
//             .bucket(
//                 "interval_10__limit_2",
//                 RateLimiterBucket::default()
//                     .interval(Duration::from_secs(10))
//                     .limit(2),
//             )
//             .start();
//
//         for _i in 1..=8 {
//             let task = rate_limiter
//                 .task(
//                     spot_api
//                         .client
//                         .get("/0/public/Assets")
//                         .unwrap()
//                         .try_query_arg("pairs", &None::<&str>)
//                         .unwrap()
//                         .try_query_arg("info", &None::<&str>)
//                         .unwrap(),
//                 )
//                 .cost("interval_1__limit_1", 1)
//                 .cost("interval_10__limit_2", 1)
//                 .send::<AssetInfoResponse>();
//
//             assert_eq!(task.metadata().costs.get("interval_1__limit_1"), Some(&1));
//             assert_eq!(task.metadata().costs.get("interval_10__limit_2"), Some(&1));
//         }
//     }
//
//     #[actix_rt::test]
//     async fn test_rate_limiter_delay() {
//         let signer = ApiCred::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let tier = RateLimiterTier::Starter;
//         let spot_api = SpotApi::new(signer, proxy, tier);
//
//         let rate_limiter = RateLimiterBuilder::default()
//             .bucket(
//                 "delay_10__interval_1__limit_1",
//                 RateLimiterBucket::default()
//                     .delay(Duration::from_secs(3))
//                     .interval(Duration::from_secs(3))
//                     .limit(1),
//             )
//             .start();
//
//         let instant = Instant::now();
//         for _i in 1..=2 {
//             let task_res = rate_limiter
//                 .task(
//                     spot_api
//                         .client
//                         .get("/0/public/Assets")
//                         .unwrap()
//                         .try_query_arg("pairs", &None::<&str>)
//                         .unwrap()
//                         .try_query_arg("info", &None::<&str>)
//                         .unwrap(),
//                 )
//                 .cost("delay_10__interval_1__limit_1", 1)
//                 .send::<AssetInfoResponse>()
//                 .await;
//
//             assert!(task_res.is_ok());
//         }
//
//         assert!(instant.elapsed() >= Duration::from_secs(6));
//     }
//
//     #[actix_rt::test]
//     async fn test_rate_limiter_wrong_bucket() {
//         let signer = ApiCred::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let tier = RateLimiterTier::Starter;
//         let spot_api = SpotApi::new(signer, proxy, tier);
//
//         let rate_limiter = RateLimiterBuilder::default()
//             .bucket(
//                 "delay_10__interval_1__limit_1",
//                 RateLimiterBucket::default()
//                     .delay(Duration::from_secs(10))
//                     .interval(Duration::from_secs(10))
//                     .limit(1),
//             )
//             .start();
//
//         let task_res = rate_limiter
//             .task(
//                 spot_api
//                     .client
//                     .get("/0/public/Assets")
//                     .unwrap()
//                     .try_query_arg("pairs", &None::<&str>)
//                     .unwrap()
//                     .try_query_arg("info", &None::<&str>)
//                     .unwrap(),
//             )
//             .cost("interval_1__limit_1", 1)
//             .send::<AssetInfoResponse>()
//             .await;
//         assert!(task_res.is_err())
//     }
//
//     #[actix_rt::test]
//     async fn test_rate_limiter_decrease() {
//         let signer = ApiCred::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let tier = RateLimiterTier::Starter;
//         let spot_api = SpotApi::new(signer, proxy, tier);
//
//         let rate_limiter = RateLimiterBuilder::default()
//             .bucket(
//                 "interval_3__limit_5",
//                 RateLimiterBucket::default()
//                     .mode(RateLimiterBucketMode::CoinbaseDecrease)
//                     .interval(Duration::from_secs(3))
//                     .limit(5),
//             )
//             .start();
//
//         let instant = Instant::now();
//         for _i in 1..10 {
//             let _task_res = rate_limiter
//                 .task(
//                     spot_api
//                         .client
//                         .get("/0/public/Assets")
//                         .unwrap()
//                         .try_query_arg("pairs", &None::<&str>)
//                         .unwrap()
//                         .try_query_arg("info", &None::<&str>)
//                         .unwrap(),
//                 )
//                 .cost("interval_3__limit_5", 1)
//                 .send::<AssetInfoResponse>()
//                 .await;
//         }
//
//         assert!(instant.elapsed() >= Duration::from_secs(13));
//     }
//
//     #[actix_rt::test]
//     async fn test_rate_limiter_priority() {
//         let signer = ApiCred::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let proxy = Proxy::from_env_with_prefix(CCX_COINBASE_API_PREFIX);
//         let tier = RateLimiterTier::Starter;
//         let spot_api = SpotApi::new(signer, proxy, tier);
//
//         let rate_limiter = RateLimiterBuilder::default()
//             .bucket(
//                 "interval_3__limit_5",
//                 RateLimiterBucket::default()
//                     .mode(RateLimiterBucketMode::CoinbaseDecrease)
//                     .interval(Duration::from_secs(3))
//                     .limit(5),
//             )
//             .start();
//
//         let instant = Instant::now();
//         let counter = Arc::new(AtomicU8::new(0));
//         let position = Arc::new(AtomicU8::new(0));
//         {
//             let counter = counter.clone();
//             let position = position.clone();
//             let rate_limiter = rate_limiter.clone();
//             let spot_api = spot_api.clone();
//             actix::spawn(async move {
//                 while counter.load(Ordering::SeqCst) < 6 {
//                     sleep(Duration::from_millis(10)).await;
//                 }
//
//                 let _task_res = rate_limiter
//                     .task(
//                         spot_api
//                             .client
//                             .get("/0/public/Assets")
//                             .unwrap()
//                             .try_query_arg("pairs", &None::<&str>)
//                             .unwrap()
//                             .try_query_arg("info", &None::<&str>)
//                             .unwrap(),
//                     )
//                     .cost("interval_3__limit_5", 1)
//                     .priority(1)
//                     .send::<AssetInfoResponse>()
//                     .await;
//                 println!(
//                     "Time now: {:?}",
//                     std::time::SystemTime::now()
//                         .duration_since(std::time::SystemTime::UNIX_EPOCH)
//                         .unwrap()
//                         .as_secs()
//                 );
//                 let current = counter.fetch_add(1, Ordering::SeqCst) + 1;
//                 position.store(current, Ordering::SeqCst);
//                 println!("PRIORITY POS: {}", current);
//             });
//         }
//
//         for _ in 1..10 {
//             let counter = counter.clone();
//             let rate_limiter = rate_limiter.clone();
//             let spot_api = spot_api.clone();
//             actix::spawn(async move {
//                 let _task_res = rate_limiter
//                     .task(
//                         spot_api
//                             .client
//                             .get("/0/public/Assets")
//                             .unwrap()
//                             .try_query_arg("pairs", &None::<&str>)
//                             .unwrap()
//                             .try_query_arg("info", &None::<&str>)
//                             .unwrap(),
//                     )
//                     .cost("interval_3__limit_5", 1)
//                     .send::<AssetInfoResponse>()
//                     .await;
//                 println!(
//                     "Time now: {:?}",
//                     std::time::SystemTime::now()
//                         .duration_since(std::time::SystemTime::UNIX_EPOCH)
//                         .unwrap()
//                         .as_secs()
//                 );
//                 let current = counter.fetch_add(1, Ordering::SeqCst) + 1;
//                 println!("TASK POS: {}", current);
//             });
//         }
//
//         while counter.load(Ordering::SeqCst) < 10 {
//             sleep(Duration::from_millis(100)).await;
//         }
//
//         assert!((7..=8).contains(&position.load(Ordering::SeqCst)));
//         assert!(instant.elapsed() >= Duration::from_secs(13));
//     }
// }
