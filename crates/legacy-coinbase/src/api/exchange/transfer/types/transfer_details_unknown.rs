use crate::api::exchange::prelude::*;

/// Additional details about a transfer.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UnknownDetails {
    /// The unique identifier of the Coinbase account associated with the transfer.
    #[serde(default, with = "maybe_str")]
    pub coinbase_account_id: Option<Uuid>,
    /// The unique identifier of the Coinbase transaction associated with the transfer.
    #[serde(default, with = "maybe_str")]
    pub coinbase_transaction_id: Option<String>,
    /// The unique identifier of the Coinbase payment method associated with the transfer (if applicable).
    #[serde(default, with = "maybe_str")]
    pub coinbase_payment_method_id: Option<Uuid>,
}
