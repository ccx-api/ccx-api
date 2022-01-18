use super::prelude::*;

mod filter;
mod rate_limit;
mod symbol;

pub use self::filter::*;
pub use self::rate_limit::*;
pub use self::symbol::*;

pub const FAPI_V1_EXCHANGE_INFO: &str = "/fapi/v1/exchangeInfo";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimit>,
    /// Ignore please.
    /// If you want to check current server time, please check via `UsdtmApi::time()`.
    pub server_time: u64,
    pub assets: Vec<Asset>,
    pub symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeFilter {}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: Atom,
    /// Whether the asset can be used as margin in Multi-Assets mode.
    pub margin_available: bool,
    /// Auto-exchange threshold in Multi-Assets margin mode
    pub auto_asset_exchange: Option<Decimal>,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<Signer: crate::client::BinaneSigner> UmApi<Signer> {
        /// Current exchange trading rules and symbol information.
        ///
        /// Weight: 1
        pub async fn exchange_info(&self) -> BinanceResult<ExchangeInformation> {
            self.client.get(FAPI_V1_EXCHANGE_INFO)?.send().await
        }
    }
}
