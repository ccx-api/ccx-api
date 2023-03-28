use crate::api::exchange::currency::SupportedNetworkStatus;
use crate::api::exchange::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct SupportedNetwork {
    pub id: Atom,
    pub name: Atom,
    pub status: SupportedNetworkStatus,
    // pub status: Atom,
    pub contract_address: Atom,
    pub crypto_address_link: Atom,
    pub crypto_transaction_link: Atom,
    pub min_withdrawal_amount: f64,
    pub max_withdrawal_amount: f64,
    pub network_confirmations: u32,
    pub processing_time_seconds: Option<u32>,
}
