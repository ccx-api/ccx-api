use crate::api::exchange::prelude::*;
use crate::api::exchange::TransferDetails;
use crate::api::exchange::TransferType;

/// Represents a transfer object from Coinbase Exchange/Pro API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transfer {
    /// The unique identifier of the transfer.
    pub id: Uuid,
    /// The type of the transfer (deposit, withdraw, internal_deposit, or internal_withdraw).
    pub r#type: TransferType,
    /// The time at which the transfer was created.
    pub created_at: DtCoinbase,
    /// The time at which the transfer was completed.
    pub completed_at: DtCoinbase,
    /// The time at which the transfer was canceled (if applicable).
    pub canceled_at: Option<DtCoinbase>,
    /// The time at which the transfer was processed (if applicable).
    pub processed_at: Option<DtCoinbase>,
    /// A nonce assigned by the user for their own reference.
    pub user_nonce: Option<i64>,
    /// The amount of the transfer, as a decimal.
    pub amount: Decimal,
    /// Additional details about the transfer.
    pub details: TransferDetails,
}
