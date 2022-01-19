use crate::MerchantId;
use ccx_api_lib::ApiCred;

use crate::client::BinancePaySigner;
use crate::client::Config;
use crate::client::RestClient;
use crate::client::CCX_BINANCE_PAY_API_PREFIX;

mod certificates;
mod close_order;
mod create_order;
mod query_order;
mod transfer_fund;
mod webhook;

pub mod prelude {
    pub use super::certificates::Certificate;
    pub use super::certificates::CertificateRequest;
    pub use super::certificates::CertificateResponse;
    pub use super::close_order::CloseOrderRequest;
    pub use super::close_order::CloseOrderResponse;
    pub use super::create_order::CreateOrderRequest;
    pub use super::create_order::CreateOrderResponse;
    pub use super::create_order::OrderResult;
    pub use super::query_order::PayerInfo;
    pub use super::query_order::QueryOrderRequest;
    pub use super::query_order::QueryOrderResponse;
    pub use super::query_order::QueryOrderResult;
    pub use super::transfer_fund::TransferFundRequest;
    pub use super::transfer_fund::TransferFundResponse;
    pub use super::transfer_fund::TransferResult;
    pub use super::transfer_fund::TransferStatus;
    pub use super::transfer_fund::TransferType;
    pub use super::webhook::BinancePayWebHookRequest;
    pub use super::webhook::BinancePayWebHookResponse;
    pub use super::webhook::BizStatus;
    pub use super::webhook::Notification;
    pub use super::webhook::ReturnCode;
}

#[derive(Clone)]
pub struct Api<S>
where
    S: BinancePaySigner,
{
    pub client: RestClient<S>,
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub fn new(signer: S, testnet: bool, merchant_id: MerchantId) -> Api<S> {
        Api::with_config(Config::new(signer, testnet, merchant_id))
    }

    pub fn from_env() -> Api<ApiCred> {
        Self::from_env_with_prefix(CCX_BINANCE_PAY_API_PREFIX)
    }

    pub fn from_env_with_prefix(prefix: &str) -> Api<ApiCred> {
        let testnet = Config::<S>::env_var("TESTNET").as_deref() == Some("1");
        let merchant_id = MerchantId::from_env_with_prefix(prefix);
        Api::new(ApiCred::from_env_with_prefix(prefix), testnet, merchant_id)
    }

    pub fn with_config(config: Config<S>) -> Api<S> {
        let client = RestClient::with_config(config);
        Api { client }
    }

    pub fn merchant_id(&self) -> u64 {
        self.client.merchant_id()
    }
}

pub mod json_string {
    use serde::de::{self, Deserialize, DeserializeOwned, Deserializer};
    use serde::ser::{self, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let j = serde_json::to_string(value).map_err(ser::Error::custom)?;
        j.serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        let j = String::deserialize(deserializer)?;
        serde_json::from_str(&j).map_err(de::Error::custom)
    }
}

pub mod uuid_simple {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(value: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_simple().to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        Uuid::deserialize(deserializer)
    }
}

pub mod opt_uuid_simple {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(value: &Option<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(uuid) => uuid.to_simple().to_string().serialize(serializer),
            None => Option::<String>::None.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<Uuid>::deserialize(deserializer)
    }
}
