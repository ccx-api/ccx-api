use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::BaseAmount;
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;

// Re-export shared types from types::transfer module
pub use crate::types::transfer::{
    Transfer, TransferCoinSpecific, TransferEntry, TransferHistoryEntry, TransferState,
    TransferStatus, TransferType, UtxoEntry,
};

/// Transaction type for BitGo send operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    /// Speed up transactions with a certain nonce by adjusting the gas setting
    #[serde(rename = "acceleration")]
    Acceleration,

    /// XRP AccountSet transactions
    #[serde(rename = "accountSet")]
    AccountSet,

    /// Solana enable token transactions
    #[serde(rename = "enabletoken")]
    EnableToken,

    /// Stacks staking lock transactions
    #[serde(rename = "stakingLock")]
    StakingLock,

    /// Stacks staking unlock transactions
    #[serde(rename = "stakingUnlock")]
    StakingUnlock,

    /// Native-asset transfers (most common)
    #[serde(rename = "transfer")]
    Transfer,

    /// Token transfers
    #[serde(rename = "transfertoken")]
    TransferToken,

    /// Stellar trustline transactions
    #[serde(rename = "trustline")]
    Trustline,

    /// AVAX add validator
    #[serde(rename = "addValidator")]
    AddValidator,

    /// AVAX export transaction
    #[serde(rename = "export")]
    Export,

    /// AVAX import transaction
    #[serde(rename = "import")]
    Import,

    /// XRP payment transaction (default for XRP)
    #[serde(rename = "payment")]
    Payment,
}

/// Transaction EIP-1559 settings for Ethereum
#[apply(Request)]
pub struct Eip1559Settings {
    /// Max priority fee per gas
    max_priority_fee_per_gas: Option<BaseAmount>,

    /// Max fee per gas
    max_fee_per_gas: Option<BaseAmount>,
}

/// Transaction memo for various coins
#[apply(Request)]
pub struct TransactionMemo {
    /// Type of memo
    #[serde(rename = "type")]
    memo_type: Option<String>,

    /// Value of the memo
    value: Option<String>,
}

/// Trustline definition for Stellar
#[apply(Request)]
pub struct Trustline {
    /// Token identifier
    token: String,

    /// Action to perform (add or remove)
    action: String,

    /// Limit amount in base units (stroops)
    limit: Option<BaseAmount>,
}

/// Staking options for CSPR and STX
#[apply(Request)]
pub struct StakingOptions {
    /// Amount to stake or unstake in base units
    amount: Option<BaseAmount>,

    /// Validator address used to delegate or undelegate (required for CSPR)
    validator: Option<String>,
}

/// Transaction reservation settings
#[apply(Request)]
pub struct ReservationSettings {
    /// Expiration time for the unspent reservations
    expire_time: String,
}

/// Request to send coins from a wallet
#[apply(Request)]
pub struct SendCoins {
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    wallet_id: String,

    /// Target address
    address: String,

    /// Amount in base units (e.g., satoshis, wei, drops, stroops)
    amount: BaseAmount,

    /// Wallet passphrase to decrypt the user key
    wallet_passphrase: Option<String>,

    /// Private key in string form (optional, if wallet_passphrase is not available)
    prv: Option<String>,

    /// Transaction type (required for MPC wallets)
    #[serde(rename = "type")]
    tx_type: Option<TransactionType>,

    /// Number of blocks required to confirm a transaction (BTC only)
    num_blocks: Option<u32>,

    /// Custom fee rate per kilobyte in base units
    fee_rate: Option<BaseAmount>,

    /// Maximum fee rate per kilobyte in base units (BTC only)
    max_fee_rate: Option<BaseAmount>,

    /// Fee multiplier for UTXO coins
    fee_multiplier: Option<BaseAmount>,

    /// Minimum confirmations for unspents
    min_confirms: Option<u32>,

    /// Enforce min confirms for change outputs
    enforce_min_confirms_for_change: Option<bool>,

    /// Custom gas price for ETH and ERC20 tokens
    gas_price: Option<BaseAmount>,

    /// EIP-1559 settings for ETH transactions
    eip1559: Option<Eip1559Settings>,

    /// Custom gas limit for ETH and ERC20 tokens
    gas_limit: Option<String>,

    /// Minimum number of good-sized unspents to maintain
    target_wallet_unspents: Option<u32>,

    /// Ignore unspents smaller than this amount
    min_value: Option<BaseAmount>,

    /// Ignore unspents larger than this amount
    max_value: Option<BaseAmount>,

    /// Unique identifier for the transaction (prevents double-sending)
    sequence_id: Option<String>,

    /// Nonce value (DOT only)
    nonce: Option<String>,

    /// Disable automatic change splitting
    no_split_change: Option<bool>,

    /// Explicitly specify unspents to use
    unspents: Option<Vec<String>>,

    /// Custom address for change outputs
    change_address: Option<String>,

    /// Format of the returned transaction hex
    tx_format: Option<String>,

    /// Use Dash InstantSend feature
    instant: Option<bool>,

    /// Transaction memo for CSPR, EOS, HBAR, RUNE, STX, TON, XLM, and XRP
    memo: Option<TransactionMemo>,

    /// Optional comment for the transaction (only stored in BitGo)
    comment: Option<String>,

    /// Destination chain for AVAX import/export
    destination_chain: Option<String>,

    /// Source chain for AVAX import/export
    source_chain: Option<String>,

    /// DEPRECATED: Address type for change
    address_type: Option<String>,

    /// Address type for change
    change_address_type: Option<String>,

    /// Start time for transaction validity window (HBAR only)
    start_time: Option<String>,

    /// Consolidation ID (ALGO/TEZOS only)
    consolidate_id: Option<String>,

    /// Absolute max ledger for transaction acceptance (XRP only)
    last_ledger_sequence: Option<u32>,

    /// Relative ledger height for transaction acceptance (XRP only)
    ledger_sequence_delta: Option<u32>,

    /// List of transactions to accelerate using RBF
    rbf_tx_ids: Option<Vec<String>>,

    /// Mark transaction as eligible for RBF
    is_replaceable_by_fee: Option<bool>,

    /// Optional block this transaction is valid from
    valid_from_block: Option<u32>,

    /// Optional block this transaction is valid until
    valid_to_block: Option<u32>,

    /// List of trustlines to manage (Stellar only)
    trustlines: Option<Vec<Trustline>>,

    /// Staking options for CSPR and STX
    staking_options: Option<StakingOptions>,

    /// Message key for XRP accountSet transactions
    message_key: Option<String>,

    /// Reservation settings for UTXO coins
    reservation: Option<ReservationSettings>,

    /// Optional data to pass to the transaction (ETH only)
    data: Option<String>,

    /// Set to true if funds need to come from single sig address (ETH, AVAXC, POLYGON)
    hop: Option<bool>,

    /// Token name defined in BitGoJS Statics package
    token_name: Option<String>,
}

/// Response for SendCoins request
#[apply(Response)]
pub struct SendCoinsResponse {
    /// Transfer information
    pub transfer: Transfer,

    /// Status message
    pub status: Option<TransferStatus>,

    /// Transaction ID
    pub txid: Option<String>,

    /// Raw transaction
    pub tx: Option<String>,
}

impl Response for SendCoinsResponse {}

impl Request for SendCoins {
    type Response = SendCoinsResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let coin = &self.coin;
        let wallet_id = &self.wallet_id;

        format!("/api/v2/{coin}/wallet/{wallet_id}/sendcoins").into()
    }
}

impl SignedRequest for SendCoins {}
