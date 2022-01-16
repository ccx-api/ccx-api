use std::env::var;

use exchange_sign_hook::SignClosure;
use url::Url;

use super::API_BASE;
use super::API_BASE_TESTNET;

pub static CCX_BINANCE_API_KEY: &str = "CCX_BINANCE_API_KEY";
pub static CCX_BINANCE_API_SECRET: &str = "CCX_BINANCE_API_SECRET";
pub static CCX_BINANCE_API_TESTNET: &str = "CCX_BINANCE_API_TESTNET";
pub static CCX_BINANCE_API_MERCHANT_ID: &str = "CCX_BINANCE_API_MERCHANT_ID";

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

#[derive(Clone)]
pub struct KeyClosure {
    pub(super) api_key: String,
    pub(super) closure: SignClosure,
}

impl KeyClosure {
    pub fn new(api_key: String, closure: SignClosure) -> Self {
        Self { api_key, closure }
    }
}

#[derive(Clone)]
pub enum Signer {
    Cred(ApiCred),
    Hook(KeyClosure),
}

impl From<ApiCred> for Signer {
    fn from(cred: ApiCred) -> Self {
        Signer::Cred(cred)
    }
}

impl From<KeyClosure> for Signer {
    fn from(closure: KeyClosure) -> Self {
        Signer::Hook(closure)
    }
}

/// API config.
#[derive(Clone)]
pub struct Config {
    pub signer: Signer,
    pub api_base: Url,
    pub merchant_id: MerchantId,
}

/// API credentials.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiCred {
    pub(super) key: String,
    pub(super) secret: String,
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

    pub fn from_env() -> Self {
        let cred = ApiCred::from_env();
        let testnet = var(CCX_BINANCE_API_TESTNET).unwrap_or_default() == "1";
        let merchant_id = MerchantId::from_env();
        Self::new(cred, testnet, merchant_id)
    }

    /// Reads config from env vars with names like:
    /// "${prefix}_KEY", "${prefix}_SECRET", and "${prefix}_TESTNET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        let cred = ApiCred::from_env_with_prefix(prefix);
        let testnet = dbg!(var(format!("{}_TESTNET", prefix))).unwrap_or_default() == "1";
        let merchant_id = MerchantId::from_env_with_prefix(prefix);
        Self::new(cred, testnet, merchant_id)
    }

    pub(crate) fn api_key(&self) -> String {
        match self.signer {
            Signer::Cred(ref cred) => cred.key.clone(),
            Signer::Hook(ref hook) => hook.api_key.clone(),
        }
    }

    // pub(crate) api_secre(&self) -> Option<String> {

    // }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(ApiCred::default(), false, MerchantId::default())
    }
}

impl ApiCred {
    pub fn new(key: Option<String>, secret: Option<String>) -> Self {
        ApiCred {
            key: key.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }

    pub fn from_env() -> Self {
        ApiCred {
            key: var(CCX_BINANCE_API_KEY).unwrap_or_default(),
            secret: var(CCX_BINANCE_API_SECRET).unwrap_or_default(),
        }
    }

    /// Reads credentials from env vars with names like:
    /// "${prefix}_KEY" and "${prefix}_SECRET"
    pub fn from_env_with_prefix(prefix: &str) -> Self {
        ApiCred {
            key: var(format!("{}_KEY", prefix)).unwrap_or_default(),
            secret: var(format!("{}_SECRET", prefix)).unwrap_or_default(),
        }
    }
}
