use crate::api::exchange::currency::CurrencyType;
// use crate::api::exchange::currency::PushPaymentMethod;
use crate::api::exchange::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyDetails {
    pub r#type: CurrencyType,
    pub group_types: Vec<Atom>,
    #[serde(default)]
    pub symbol: Option<Atom>,
    #[serde(default)]
    pub network_confirmations: Option<u32>,
    #[serde(default)]
    pub sort_order: Option<u32>,
    #[serde(default)]
    pub crypto_address_link: Option<Atom>,
    #[serde(default)]
    pub crypto_transaction_link: Option<Atom>,
    #[serde(default)]
    pub push_payment_methods: Option<Vec<String>>,
    #[serde(default)]
    pub display_name: Option<Atom>,
    #[serde(default)]
    pub processing_time_seconds: Option<u32>,
    #[serde(default)]
    pub min_withdrawal_amount: Option<Decimal>,
    #[serde(default)]
    pub max_withdrawal_amount: Option<Decimal>,
}
