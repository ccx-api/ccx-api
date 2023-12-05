use serde::de;
use serde::Serialize;

mod balance;
mod close_order;
mod create_order;
mod error;
mod order_status;

pub use balance::*;
pub use close_order::*;
pub use create_order::*;
pub use error::*;
pub use order_status::*;

pub const API_BASE: &str = "https://openplatform.gateapi.io/";
pub const API_BASE_SANDBOX: &str = "https://miniapp-sandbox.gateapi.io/";

pub enum ApiMethod {
    Get,
    Post,
}

pub enum ApiVersion {
    V1,
    V2,
}

impl ApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "v1",
            ApiVersion::V2 => "v2",
        }
    }
}

pub trait Request: Serialize {
    const METHOD: ApiMethod;
    const VERSION: ApiVersion;
    const PATH: &'static str;

    type Response: de::DeserializeOwned;
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::GatepayApiCred;
    use ccx_api_lib::Proxy;

    pub use super::*;
    use crate::client::config::GatepayApiConfig;
    use crate::client::config::CCX_GATEPAY_API_PREFIX;
    use crate::client::rest::GatepayRestClient;
    use crate::client::rest::RequestError;
    use crate::client::signer::GatepaySigner;

    #[derive(Clone)]
    pub struct GatepayApi<S>
    where
        S: GatepaySigner,
    {
        pub client: GatepayRestClient<S>,
    }

    impl<S> GatepayApi<S>
    where
        S: GatepaySigner,
    {
        pub fn new(signer: S, is_sandbox: bool, proxy: Option<Proxy>) -> GatepayApi<S> {
            let api_base = if is_sandbox {
                API_BASE_SANDBOX.parse().unwrap()
            } else {
                API_BASE.parse().unwrap()
            };
            GatepayApi::with_config(GatepayApiConfig::new(signer, api_base, proxy))
        }

        pub fn from_env() -> GatepayApi<GatepayApiCred> {
            Self::from_env_with_prefix(CCX_GATEPAY_API_PREFIX)
        }

        pub fn from_env_with_prefix(prefix: &str) -> GatepayApi<GatepayApiCred> {
            // FIXME prefix (also in the BinancePay API backend)
            let is_sandbox = GatepayApiConfig::<S>::env_var("SANDBOX").as_deref() == Some("1");
            let proxy = Proxy::from_env_with_prefix(prefix);
            log::debug!(
                "from_env_with_prefix proxy :: {:?}",
                proxy.as_ref().map(|p| (&p.host, p.port))
            );
            GatepayApi::new(
                GatepayApiCred::from_env_with_prefix(prefix),
                is_sandbox,
                proxy,
            )
        }

        pub fn with_config(config: GatepayApiConfig<S>) -> GatepayApi<S> {
            let client = GatepayRestClient::new(config);
            GatepayApi { client }
        }

        pub async fn request<R: Request>(&self, request: &R) -> Result<R::Response, RequestError> {
            Ok(self.client.rest(request).now().sign().await?.call().await?)
        }
    }
}
