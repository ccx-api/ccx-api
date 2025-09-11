mod error;
mod method;
mod request;
pub mod spot;
mod version;
pub mod wallet;
pub mod withdrawal;

pub use error::*;
pub use method::*;
pub use request::*;
pub use version::*;

pub const API_BASE: &str = "https://api.gateio.ws/api/";
pub const STREAM_BASE: &str = "wss://api.gateio.ws/ws/v4/";

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::Proxy;
    use ref_cast::RefCast;
    use spot::SpotApi;
    use wallet::WalletApi;
    use withdrawal::WithdrawalApi;

    pub use super::*;
    use crate::client::GateSigner;
    use crate::client::config::CCX_GATE_API_PREFIX;
    use crate::client::config::GateApiConfig;
    use crate::client::rest::RequestError;
    use crate::client::rest::RestClient;
    use crate::client::websocket::WebsocketStream;
    use crate::error::GateResult;
    use crate::util::GateApiCred;

    #[derive(Clone)]
    pub struct GateApi<S> {
        pub client: RestClient<S>,
    }

    impl<S> GateApi<S> {
        pub fn new(signer: S, proxy: Option<Proxy>) -> GateApi<S> {
            let api_base = API_BASE.parse().unwrap();
            let stream_base = STREAM_BASE.parse().unwrap();
            GateApi::with_config(GateApiConfig::new(signer, api_base, stream_base, proxy))
        }

        pub fn from_env() -> GateApi<GateApiCred> {
            Self::from_env_with_prefix(CCX_GATE_API_PREFIX)
        }

        pub fn from_env_with_prefix(prefix: &str) -> GateApi<GateApiCred> {
            // FIXME prefix
            let proxy = Proxy::from_env_with_prefix(prefix);
            log::debug!(
                "from_env_with_prefix proxy :: {:?}",
                proxy.as_ref().map(|p| (&p.host, p.port))
            );
            GateApi::new(GateApiCred::from_env_with_prefix(prefix), proxy)
        }

        pub fn with_config(config: GateApiConfig<S>) -> GateApi<S> {
            let client = RestClient::new(config);
            GateApi { client }
        }

        /// Unsigned request. For signed see [Self::signed_request]
        pub async fn request<R: PublicRequest>(
            &self,
            path: &str,
            request: &R,
        ) -> Result<R::Response, RequestError> {
            Ok(self
                .client
                .prepare_rest(path, request)
                .call_unsigned()
                .await?)
        }

        /// Spot trading
        pub fn spot(&self) -> &SpotApi<S> {
            RefCast::ref_cast(self)
        }

        /// Wallet operations
        pub fn wallet(&self) -> &WalletApi<S> {
            RefCast::ref_cast(self)
        }

        /// Withdrawal operations
        pub fn withdrawal(&self) -> &WithdrawalApi<S> {
            RefCast::ref_cast(self)
        }

        pub async fn websocket(&self) -> GateResult<WebsocketStream> {
            self.client.websocket().await
        }
    }

    impl<S: GateSigner> GateApi<S> {
        pub async fn signed_request<R: PrivateRequest>(
            &self,
            path: &str,
            request: &R,
        ) -> Result<R::Response, RequestError> {
            let signed = self
                .client
                .prepare_rest(path, request)
                .with_current_timestamp()
                .sign()
                .await?;
            Ok(signed.call().await?)
        }
    }
}
