//! Transfer-related types shared across BitGo API modules
//!
//! This module contains common data structures used for transfer operations,
//! including transfer states, types, and related data structures.

use derive_more::Display;
use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};

use crate::prelude::Coin;
use crate::types::BaseAmount;
use crate::types::derive::Response;

/// Transfer state enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "camelCase")]
pub enum TransferState {
    /// Transaction is confirmed on chain
    #[serde(rename = "confirmed")]
    Confirmed,

    /// Transaction failed
    #[serde(rename = "failed")]
    Failed,

    /// Transaction is initialized (first state)
    #[serde(rename = "initialized")]
    Initialized,

    /// Transaction is pending approval
    #[serde(rename = "pendingApproval")]
    PendingApproval,

    /// Transaction was rejected by an approver
    #[serde(rename = "rejected")]
    Rejected,

    /// Transaction was reorganized from the mempool
    #[serde(rename = "removed")]
    Removed,

    /// Transaction was replaced with higher fees
    #[serde(rename = "replaced")]
    Replaced,

    /// Transaction is signed and pending confirmation
    #[serde(rename = "signed")]
    Signed,

    /// Transaction is pending on-chain confirmation
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
}

/// Transfer type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransferType {
    /// Sending transfer
    #[serde(rename = "send")]
    Send,

    /// Receiving transfer
    #[serde(rename = "receive")]
    Receive,
}

/// Sort key for transfers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransferSortBy {
    /// Sort by height ID
    #[serde(rename = "heightId")]
    HeightId,

    /// Sort by transfer ID
    #[serde(rename = "id")]
    Id,
}

/// Transfer status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransferStatus {
    /// Transaction has been signed
    #[serde(rename = "signed")]
    Signed,

    /// Transaction has been signed but notifications are suppressed
    #[serde(rename = "signed (suppressed)")]
    SignedSuppressed,

    /// Transaction is waiting for approval
    #[serde(rename = "pendingApproval")]
    PendingApproval,
}

/// Entry in a transaction
#[apply(Response)]
pub struct TransferEntry {
    /// Address affected by this Transfer
    pub address: String,

    /// Wallet ID (only visible when the user has access to the wallet)
    pub wallet: Option<String>,

    /// Value in base units as string
    #[serde(rename = "valueString")]
    pub value: BaseAmount,

    /// Whether this is a change output (only for UTXO coins and sent transfers)
    pub is_change: Option<bool>,

    /// Whether this address is the BitGo PayGo wallet
    pub is_pay_go: Option<bool>,

    /// Token symbol, if this is a token entry
    pub token: Option<String>,

    /// User provided comment about the address
    pub comment: Option<String>,

    /// User provided wallet label the address belongs to
    pub wallet_label: Option<String>,

    /// Names of addresses given by the user
    pub label: Option<String>,

    /// True if this entry is failed
    pub failed: Option<bool>,

    /// The native coin receive address associated with the solana ATA address
    /// This is currently present only for BitGo addresses
    pub associated_native_coin_address: Option<String>,
}

/// Transaction history entry
#[apply(Response)]
pub struct TransferHistoryEntry {
    /// Date of the event
    pub date: String,

    /// User who performed the action
    pub user: Option<String>,

    /// Action performed
    pub action: String,

    /// Optional comment
    pub comment: Option<String>,
}

/// Coin-specific data for transfer
#[apply(Response)]
pub struct TransferCoinSpecific {
    /// Transaction-specific data for the particular coin
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// UTXO Input/Output structure
#[apply(Response)]
pub struct UtxoEntry {
    /// Unique identifier
    pub id: String,

    /// Address
    pub address: String,

    /// Value in base units as string
    #[serde(rename = "valueString")]
    pub value: BaseAmount,

    /// Block height
    pub block_height: Option<i64>,

    /// Date of the entry
    pub date: String,

    /// Whether this is a coinbase transaction
    pub coinbase: Option<bool>,

    /// Wallet ID
    pub wallet: Option<String>,

    /// Source wallet ID
    pub from_wallet: Option<String>,

    /// Chain index
    pub chain: Option<u32>,

    /// Index
    pub index: Option<u32>,

    /// Redeem script
    pub redeem_script: Option<String>,

    /// Witness script
    pub witness_script: Option<String>,

    /// Whether this is a SegWit transaction
    pub is_segwit: Option<bool>,
}

/// Transfer information
#[apply(Response)]
pub struct Transfer {
    /// Coin symbol
    pub coin: Coin,

    /// Transfer ID
    pub id: String,

    /// Wallet ID
    pub wallet: String,

    /// Wallet type (e.g., "hot", "cold")
    pub wallet_type: Option<String>,

    /// Enterprise ID
    pub enterprise: Option<String>,

    /// Organization ID
    pub organization: Option<String>,

    /// Transaction ID on blockchain
    pub txid: String,

    /// Type of the transaction ID
    pub txid_type: Option<String>,

    /// Transaction request ID
    pub tx_request_id: Option<String>,

    /// Block height
    pub height: i64,

    /// Unique height ID of the block
    pub height_id: Option<String>,

    /// Last updated date
    pub date: String,

    /// Number of confirmations
    pub confirmations: Option<i64>,

    /// Type of transfer (send or receive)
    #[serde(rename = "type")]
    pub transfer_type: String,

    /// Total value in base units as string
    #[serde(rename = "valueString")]
    pub value: BaseAmount,

    /// Intended value string (preserved after failed transactions)
    #[serde(rename = "intendedValueString")]
    pub intended_value: Option<BaseAmount>,

    /// Base value excluding fees as string (deprecated)
    #[serde(rename = "baseValueString")]
    pub base_value: Option<BaseAmount>,

    /// Base value excluding fees as string
    #[serde(rename = "baseValueWithoutFeesString")]
    pub base_value_without_fees: Option<BaseAmount>,

    /// Fee in base units as string
    #[serde(rename = "feeString")]
    pub fee: Option<BaseAmount>,

    /// BitGo fee in base units as string
    #[serde(rename = "payGoFeeString")]
    pub pay_go_fee: Option<BaseAmount>,

    /// USD equivalent amount
    pub usd: Option<f64>,

    /// USD exchange rate at creation time
    pub usd_rate: Option<f64>,

    /// Transaction state
    pub state: TransferState,

    /// Whether this is an instant transaction
    pub instant: Option<bool>,

    /// Whether this is a reward transaction
    pub is_reward: Option<bool>,

    /// Whether this is an unlock transaction
    pub is_unlock: Option<bool>,

    /// Whether this is a fee transaction
    pub is_fee: Option<bool>,

    /// Whether sender information is verified
    pub sender_information_verified: Option<bool>,

    /// Tags associated with the transfer
    pub tags: Option<Vec<String>>,

    /// History log of the transfer
    pub history: Vec<TransferHistoryEntry>,

    /// Date when the transaction was signed
    pub signed_date: Option<String>,

    /// User comment
    pub comment: Option<String>,

    /// Metadata associated with the transfer
    pub metadata: Option<Vec<serde_json::Value>>,

    /// Time when the transaction was commented
    pub commented_time: Option<String>,

    /// Time when the transaction was signed
    pub signed_time: Option<String>,

    /// Time when the transaction was created
    pub created_time: Option<String>,

    /// Virtual size of the transaction
    pub v_size: Option<i64>,

    /// Coin-specific data
    pub coin_specific: Option<TransferCoinSpecific>,

    /// Sequence ID
    pub sequence_id: Option<String>,

    /// Address balance changes from this transfer
    pub entries: Option<Vec<TransferEntry>>,

    /// Whether users were notified
    pub users_notified: Option<bool>,

    /// Address labels
    pub label: Option<String>,

    /// Transaction IDs that this transfer replaces
    pub replaces: Option<Vec<String>>,

    /// Transaction IDs that replace this transfer
    pub replaced_by: Option<Vec<String>>,

    /// Unique transaction identifier (sometimes redundant with txid)
    pub tx_id: Option<String>,

    /// Encoded transaction hex or base64
    pub tx: Option<String>,

    /// Transfer status
    pub status: Option<TransferStatus>,

    /// UTXO inputs (for UTXO coins)
    pub inputs: Option<Vec<UtxoEntry>>,

    /// UTXO outputs (for UTXO coins)
    pub outputs: Option<Vec<UtxoEntry>>,
}
