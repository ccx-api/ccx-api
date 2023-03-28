use crate::api::exchange::currency::CurrencyType;
use crate::api::exchange::currency::PushPaymentMethod;
use crate::api::exchange::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyDetails {
    pub r#type: CurrencyType,
    pub symbol: Option<Atom>,
    pub network_confirmations: Option<u32>,
    pub sort_order: u32,
    pub crypto_address_link: Option<Atom>,
    pub crypto_transaction_link: Option<Atom>,
    pub push_payment_methods: Option<Vec<PushPaymentMethod>>,
    pub group_types: Vec<Atom>,
    pub display_name: Option<Atom>,
    pub processing_time_seconds: Option<u32>,
    pub min_withdrawal_amount: Option<Decimal>,
    pub max_withdrawal_amount: Option<Decimal>,
}
