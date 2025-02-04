use super::super::RL_WEIGHT_PER_MINUTE;
use super::prelude::*;
use crate::client::Task;

pub const FAPI_V1_PING: &str = "/fapi/v1/ping";

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pong {}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> UmApi<S>
    where
        S: crate::client::BinanceSigner,
        S: Unpin + 'static,
    {
        /// Test connectivity to the Rest API.
        ///
        /// Weight: 1
        pub fn ping(&self) -> BinanceResult<Task<Pong>> {
            Ok(self
                .rate_limiter
                .task(self.client.get(FAPI_V1_PING)?)
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }
    }
}
