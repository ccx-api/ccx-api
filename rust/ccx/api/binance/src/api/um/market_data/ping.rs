use super::prelude::*;

pub const FAPI_V1_PING: &str = "/fapi/v1/ping";

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pong {}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<Signer: crate::client::BinanceSigner> UmApi<Signer> {
        /// Test connectivity to the Rest API.
        ///
        /// Weight: 1
        pub async fn ping(&self) -> BinanceResult<Pong> {
            self.client.get(FAPI_V1_PING)?.send().await
        }
    }
}
