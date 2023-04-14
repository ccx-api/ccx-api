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
    pub min_withdrawal_amount: Option<f64>,
    #[serde(default)]
    pub max_withdrawal_amount: Option<f64>,
    #[serde(default)]
    pub network_confirmations: Option<u32>,
    #[serde(default)]
    pub processing_time_seconds: Option<u32>,
}
