use ccx_api_lib::ApiCred;
use ccx_api_lib::Proxy;

use crate::client::BinancePaySigner;
use crate::client::Config;
use crate::client::RestClient;
use crate::client::CCX_BINANCE_PAY_API_PREFIX;
use crate::MerchantId;

mod certificates_v1;
mod certificates_v2;
mod close_order_v1;
mod close_order_v2;
mod create_order_v1;
mod create_order_v2;
mod query_order_v1;
mod query_order_v2;
mod response;
mod submerchant_add_v1;
mod submerchant_modify_v1;
mod transfer_fund_v1;
mod transfer_fund_v2;
mod webhook;

pub mod prelude {
    pub use super::certificates_v1::V1CertificateRequest;
    pub use super::certificates_v2::V2CertificateRequest;
    pub use super::close_order_v1::V1CloseOrderRequest;
    pub use super::close_order_v2::V2CloseOrderRequest;
    pub use super::create_order_v1::V1CreateOrderRequest;
    pub use super::create_order_v1::V1OrderResult;
    pub use super::create_order_v2::V2CreateOrderRequest;
    pub use super::create_order_v2::V2OrderResult;
    pub use super::query_order_v1::PayerInfo;
    pub use super::query_order_v1::V1QueryOrderRequest;
    pub use super::query_order_v1::V1QueryOrderResult;
    pub use super::query_order_v2::V2QueryOrderRequest;
    pub use super::query_order_v2::V2QueryOrderResult;
    pub use super::response::BinancePayResponse;
    pub use super::submerchant_add_v1::V1SubmerchantAddRequest;
    pub use super::submerchant_add_v1::V1SubmerchantAddResult;
    pub use super::submerchant_modify_v1::V1SubmerchantModifyRequest;
    pub use super::submerchant_modify_v1::V1SubmerchantModifyResult;
    pub use super::transfer_fund_v1::V1TransferFundRequest;
    pub use super::transfer_fund_v1::V1TransferResult;
    pub use super::transfer_fund_v2::V2TransferFundRequest;
    pub use super::transfer_fund_v2::V2TransferResult;
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

impl<S: BinancePaySigner> Api<S> {
    pub fn new(signer: S, testnet: bool, merchant_id: MerchantId, proxy: Option<Proxy>) -> Api<S> {
        Api::with_config(Config::new(signer, testnet, merchant_id, proxy))
    }

    pub fn from_env() -> Api<ApiCred> {
        Self::from_env_with_prefix(CCX_BINANCE_PAY_API_PREFIX)
    }

    pub fn from_env_with_prefix(prefix: &str) -> Api<ApiCred> {
        let testnet = Config::<S>::env_var("TESTNET").as_deref() == Some("1");
        let merchant_id = MerchantId::from_env_with_prefix(prefix);
        log::debug!("from_env_with_prefix merchant_id :: {:?}", merchant_id);
        let proxy = Proxy::from_env_with_prefix(prefix);
        log::debug!(
            "from_env_with_prefix proxy :: {:?}",
            proxy.as_ref().map(|p| (&p.host, p.port))
        );
        Api::new(
            ApiCred::from_env_with_prefix(prefix),
            testnet,
            merchant_id,
            proxy,
        )
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
    use serde::de::Deserialize;
    use serde::de::DeserializeOwned;
    use serde::de::Deserializer;
    use serde::de::{self};
    use serde::ser::Serialize;
    use serde::ser::Serializer;
    use serde::ser::{self};

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
    use serde::de::Deserialize;
    use serde::de::Deserializer;
    use serde::ser::Serialize;
    use serde::ser::Serializer;
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
    use serde::de::Deserialize;
    use serde::de::Deserializer;
    use serde::ser::Serialize;
    use serde::ser::Serializer;
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
