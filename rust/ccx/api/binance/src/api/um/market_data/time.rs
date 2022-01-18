use super::prelude::*;

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

    impl<Signer: crate::client::BinaneSigner> UmApi<Signer> {
        /// Test connectivity to the Rest API and get the current server time.
        ///
        /// Weight: 1
        pub async fn time(&self) -> BinanceResult<ServerTime> {
            self.client.get(FAPI_V1_TIME)?.send().await
        }
    }
}
