use std::env::var;

use ccx_api_lib::env_var_with_prefix;
use url::Url;

use crate::client::BinancePaySigner;

use super::API_BASE;
use super::API_BASE_TESTNET;

pub static CCX_BINANCE_PAY_API_PREFIX: &str = "CCX_BINANCE_PAY_API";
pub static CCX_BINANCE_API_MERCHANT_ID: &str = "CCX_BINANCE_PAY_API_MERCHANT_ID";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MerchantId(u64);

impl MerchantId {
    pub fn new(merchant_id: u64) -> Self {
        Self(merchant_id)
    }

    pub fn from_env() -> Self {
        let merchant_id = var(CCX_BINANCE_API_MERCHANT_ID).unwrap_or_default();
        let merchant_id = merchant_id.parse::<u64>().unwrap_or_default();
        Self::new(merchant_id)
    }

    pub fn from_env_with_prefix(prefix: &str) -> Self {
        let merchant_id = dbg!(var(format!("{}_MERCHANT_ID", prefix))).unwrap_or_default();
        let merchant_id = merchant_id.parse::<u64>().unwrap_or_default();
        Self::new(merchant_id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

/// API config.
#[derive(Clone)]
pub struct Config<S: BinancePaySigner> {
    pub signer: S,
    pub api_base: Url,
    pub merchant_id: MerchantId,
}

impl<S> Config<S>
where
    S: BinancePaySigner,
{
    pub fn new(signer: S, testnet: bool, merchant_id: MerchantId) -> Self {
        let api_base = if testnet {
            Url::parse(API_BASE_TESTNET).unwrap()
        } else {
            Url::parse(API_BASE).unwrap()
        };
        Config {
            signer,
            api_base,
            merchant_id,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_BINANCE_PAY_API_PREFIX, postfix)
    }

    pub(crate) fn api_key(&self) -> &str {
        self.signer.api_key()
    }

    pub(crate) fn signer(&self) -> &S {
        &self.signer
    }
}
