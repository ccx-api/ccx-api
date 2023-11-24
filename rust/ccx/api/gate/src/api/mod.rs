mod error;
mod method;
mod request;
pub mod spot;
mod version;

pub use error::*;
pub use method::*;
pub use request::*;
pub use version::*;

pub const API_BASE: &str = "https://api.gateio.ws/api/";
pub const API_FUTURES: &str = "https://fx-api.gateio.ws/api/";
pub const API_FUTURES_TESTNET: &str = "https://fx-api-testnet.gateio.ws/api/";

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::GateApiCred;
    use ccx_api_lib::Proxy;

    pub use super::*;
    use crate::client::config::GateApiConfig;
    use crate::client::config::CCX_GATE_API_PREFIX;
    use crate::client::rest::GateRestClient;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;

    #[derive(Clone)]
    pub struct GateApi<S>
    where
        S: GateSigner,
    {
        pub client: GateRestClient<S>,
    }

    pub enum GateApiType {
        Spot,
        Futures,
        FuturesTestnet,
    }

    impl<S> GateApi<S>
    where
        S: GateSigner,
    {
        pub fn new(signer: S, api_type: GateApiType, proxy: Option<Proxy>) -> GateApi<S> {
            let api_base = match api_type {
                GateApiType::Spot => API_BASE,
                GateApiType::Futures => API_FUTURES,
                GateApiType::FuturesTestnet => API_FUTURES_TESTNET,
            };
            let api_base = api_base.parse().unwrap();
            GateApi::with_config(GateApiConfig::new(signer, api_base, proxy))
        }

        pub fn from_env() -> GateApi<GateApiCred> {
            Self::from_env_with_prefix(CCX_GATE_API_PREFIX)
        }

        pub fn from_env_with_prefix(prefix: &str) -> GateApi<GateApiCred> {
            // FIXME prefix
            let api_type = match GateApiConfig::<S>::env_var("TYPE").as_deref() {
                Some("spot") => GateApiType::Spot,
                Some("futures") => GateApiType::Futures,
                Some("futures-testnet") => GateApiType::FuturesTestnet,
                None => GateApiType::Spot,
                Some(s) => {
                    log::error!("Invalid API type {s:?}");
                    GateApiType::Spot
                }
            };
            let proxy = Proxy::from_env_with_prefix(prefix);
            log::debug!(
                "from_env_with_prefix proxy :: {:?}",
                proxy.as_ref().map(|p| (&p.host, p.port))
            );
            GateApi::new(GateApiCred::from_env_with_prefix(prefix), api_type, proxy)
        }

        pub fn with_config(config: GateApiConfig<S>) -> GateApi<S> {
            let client = GateRestClient::new(config);
            GateApi { client }
        }

        pub async fn request<R: Request>(&self, request: &R) -> Result<R::Response, RequestError> {
            let resp = if R::IS_PUBLIC {
                self.client.prepare_rest(request).call_unsigned().await?
            } else {
                let signed = self.client.prepare_rest(request).now().sign().await?;
                signed.call().await?
            };
            Ok(resp)
        }
    }
}
