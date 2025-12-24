use ccx_api_lib::serde_util::f64_arbitrary_precision;

use crate::api::exchange::currency::SupportedNetworkStatus;
use crate::api::exchange::prelude::*;
use crate::util::maybe_str;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupportedNetwork {
    pub id: Atom,
    pub name: Atom,
    pub status: SupportedNetworkStatus,
    // pub status: Atom,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub contract_address: Option<Atom>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub crypto_address_link: Option<Atom>,
    #[serde(default)]
    #[serde(with = "maybe_str")]
    pub crypto_transaction_link: Option<Atom>,
    #[serde(default)]
    #[serde(deserialize_with = "f64_arbitrary_precision::deserialize_option")]
    pub min_withdrawal_amount: Option<f64>,
    #[serde(default)]
    #[serde(deserialize_with = "f64_arbitrary_precision::deserialize_option")]
    pub max_withdrawal_amount: Option<f64>,
    #[serde(default)]
    pub network_confirmations: Option<u32>,
    #[serde(default)]
    pub processing_time_seconds: Option<u32>,
}
