#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;

use crate::api::prime::prelude::*;

/// The status of a transaction
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum TransactionStatus {
    /// An Unknown Transaction status.
    #[serde(rename = "TRANSACTION_TRANSACTION_STATUS")]
    Unknown,

    /// The Transaction has been created and is awaiting Consensus approval.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_CREATED")]
    Created,

    /// The Transaction has reached User Consensus and is awaiting Coinbase Prime approval.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_REQUESTED")]
    Requested,

    /// The Transaction has been authorized by Coinbase Prime.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_APPROVED")]
    Approved,

    /// The transaction is currently processing and awaiting finalization.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_PROCESSING")]
    Processing,

    /// The transaction has confirmed on-chain and finished.
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_DONE")]
    Done,

    /// The transaction deposit has been detected and is awaiting finalization.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_IMPORT_PENDING")]
    ImportPending,

    /// The transaction deposit and reward has been detected.
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_IMPORTED")]
    Imported,

    /// The transaction has been cancelled.
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_CANCELLED")]
    Cancelled,

    /// The transaction was rejected before construction and broadcasting.
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_REJECTED")]
    Rejected,

    /// The transaction is taking longer than expected to confirm on-chain.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_DELAYED")]
    Delayed,

    /// The transaction has been recreated and retried, this occurs when network
    /// congestion results in transfers becoming extremely delayed due to insufficient
    /// fees or network resources such as CPU, RAM, or NET.
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_RETRIED")]
    Retried,

    /// The transaction failed on-chain (the fee was spent but the operation failed).
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_FAILED")]
    Failed,

    /// The transaction has expired.
    /// This is a terminal status.
    #[serde(rename = "TRANSACTION_EXPIRED")]
    Expired,

    /// The transaction has reached an OTHER status.
    /// This is a non-terminal status.
    #[serde(rename = "TRANSACTION_OTHER_STATUS")]
    Other,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(TransactionStatus);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(TransactionStatus);

impl TransactionStatus {
    pub fn is_terminal(&self) -> bool {
        match self {
            TransactionStatus::Done
            | TransactionStatus::Imported
            | TransactionStatus::Cancelled
            | TransactionStatus::Rejected
            | TransactionStatus::Retried
            | TransactionStatus::Failed
            | TransactionStatus::Expired => true,
            TransactionStatus::Unknown
            | TransactionStatus::Created
            | TransactionStatus::Requested
            | TransactionStatus::Approved
            | TransactionStatus::Processing
            | TransactionStatus::ImportPending
            | TransactionStatus::Delayed
            | TransactionStatus::Other => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for status in enum_iterator::all::<TransactionStatus>() {
            let s = serde_json::to_string(&status).unwrap();
            let status2: TransactionStatus = serde_json::from_str(&s).unwrap();
            assert_eq!(status, status2);
        }
    }
}
