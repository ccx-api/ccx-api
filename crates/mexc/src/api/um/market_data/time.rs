use super::super::RL_WEIGHT_PER_MINUTE;
use super::prelude::*;
use crate::client::Task;

pub const FAPI_V1_TIME: &str = "/fapi/v1/time";

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> UmApi<S>
    where
        S: crate::client::MexcSigner,
        S: Unpin + 'static,
    {
        /// Test connectivity to the Rest API and get the current server time.
        ///
        /// Weight: 1
        pub fn time(&self) -> MexcResult<Task<ServerTime>> {
            Ok(self
                .rate_limiter
                .task(self.client.get(FAPI_V1_TIME)?)
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }
    }
}
