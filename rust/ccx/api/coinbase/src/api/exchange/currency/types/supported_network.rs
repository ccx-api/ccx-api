use crate::api::exchange::currency::SupportedNetworkStatus;
use crate::api::exchange::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupportedNetwork {
    pub id: Atom,
    pub name: Atom,
    pub status: SupportedNetworkStatus,
    // pub status: Atom,
    #[serde(default)]
    pub contract_address: Option<Atom>,
    #[serde(default)]
    pub crypto_address_link: Option<Atom>,
    #[serde(default)]
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
