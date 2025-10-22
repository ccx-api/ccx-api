use crate::api::exchange::prelude::*;

/// Additional details about a transfer.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct WithdrawalDetails {
    /// The unique identifier of the Coinbase account associated with the transfer.
    #[serde(default, with = "maybe_str")]
    pub coinbase_account_id: Option<Uuid>,
    /// The unique identifier of the Coinbase transaction associated with the transfer.
    #[serde(default, with = "maybe_str")]
    pub coinbase_transaction_id: Option<String>,
    /// The unique identifier of the Coinbase payment method associated with the transfer (if applicable).
    #[serde(default, with = "maybe_str")]
    pub coinbase_payment_method_id: Option<Uuid>,

    /// The fee associated with the transfer, as a decimal (undocumented).
    pub fee: Decimal,
    /// The subtotal of the transfer, as a decimal (undocumented).
    pub subtotal: Decimal,
    /// The network / blockchain associated with the transfer (undocumented).
    pub network: String,
    /// The address to which the transfer was sent (undocumented).
    pub sent_to_address: String,
    /// The hash of the crypto transaction associated with the transfer (undocumented).
    pub crypto_transaction_hash: String,
    /// The supposedly unique identifier of the transaction associated with the transfer in the Coinbase service (undocumented).
    pub tx_service_transaction_id: String,
}
