use std::env::var;

use ccx_api_lib::env_var_with_prefix;
use ccx_api_lib::ApiCred;
use url::Url;

use super::SignBinancePay;
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
pub struct Config {
    pub signer: Signer,
    pub api_base: Url,
    pub merchant_id: MerchantId,
}

impl Config {
    pub fn new(signer: impl Into<Signer>, testnet: bool, merchant_id: MerchantId) -> Self {
        let api_base = if testnet {
            Url::parse(API_BASE_TESTNET).unwrap()
        } else {
            Url::parse(API_BASE).unwrap()
        };
        Config {
            signer: signer.into(),
            api_base,
            merchant_id,
        }
    }

    pub fn env_var(postfix: &str) -> Option<String> {
        env_var_with_prefix(CCX_BINANCE_PAY_API_PREFIX, postfix)
    }

    pub(crate) fn api_key(&self) -> &str {
        match self.signer {
            Signer::Cred(ref cred) => cred.key.as_str(),
            Signer::Hook(ref closure) => closure.api_key.as_str(),
        }
    }

    pub(crate) fn signer(&self) -> &Signer {
        &self.signer
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(ApiCred::default(), false, MerchantId::default())
    }
}

#[derive(Clone)]
pub struct Hook {
    pub(crate) api_key: String,
    pub(crate) closure: Box<dyn SignBinancePay>,
}

impl Hook {
    pub fn new(api_key: String, closure: Box<dyn SignBinancePay>) -> Self {
        Self { api_key, closure }
    }
}

#[derive(Clone)]
pub enum Signer {
    Cred(ApiCred),
    Hook(Hook),
}

impl From<ApiCred> for Signer {
    fn from(cred: ApiCred) -> Self {
        Signer::Cred(cred)
    }
}

impl From<Hook> for Signer {
    fn from(hook: Hook) -> Self {
        Signer::Hook(hook)
    }
}
