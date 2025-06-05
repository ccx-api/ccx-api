use std::collections::HashMap;

use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, serde_as};

use crate::prelude::Coin;
use crate::proto::Response;
use crate::types::BaseAmount;
use crate::types::derive::Response;

/// Wallet type
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WalletType {
    /// Backing wallet
    Backing,
    /// Cold wallet
    Cold,
    /// Custodial wallet
    Custodial,
    /// Custodial paired wallet
    CustodialPaired,
    /// Hot wallet
    Hot,
    /// Trading wallet
    Trading,
}

/// Wallet sub-type
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WalletSubType {
    /// Distributed custody wallet
    DistributedCustody,
    /// Paired custodial wallet
    PairedCustodial,
    /// Custodial hot wallet
    CustodialHot,
    /// Custodial cold wallet
    CustodialCold,
    /// Lightning custody wallet
    LightningCustody,
    /// Lightning self custody wallet
    LightningSelfCustody,
    /// On prem wallet
    OnPrem,
}

/// Wallet permission
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletPermission {
    /// Admin permission - add/remove wallet policies, add/remove wallet users, approve/reject
    /// wallet pending approvals, withdraw assets from wallet, generate new receive addresses,
    /// view all balances and transactions
    Admin,
    /// View permission - generate new receive addresses, view all balances and transactions
    View,
    /// Spend permission - withdraw assets from wallet, generate new receive addresses,
    /// view all balances and transactions
    Spend,
    /// Freeze permission
    Freeze,
    /// Trade permission - trade on a wallet - applicable only for trading wallets
    Trade,
}

/// Pinned wallets filter
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PinnedWallets {
    /// Only pinned wallets
    PinnedOnly,
    /// Only unpinned wallets
    PinnedExcluded,
}

/// BitGo organization
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BitGoOrg {
    #[serde(rename = "BitGo Trust")]
    BitGoTrust,
    #[serde(rename = "BitGo New York")]
    BitGoNewYork,
    #[serde(rename = "BitGo Germany")]
    BitGoGermany,
    #[serde(rename = "BitGo Switzerland")]
    BitGoSwitzerland,
    #[serde(rename = "BitGo Europe ApS")]
    BitGoEuropeApS,
    #[serde(rename = "Frankfurt DE Trust")]
    FrankfurtDETrust,
    #[serde(rename = "BitGo Singapore")]
    BitGoSingapore,
    #[serde(rename = "BitGo Korea")]
    BitGoKorea,
    #[serde(rename = "BitGo Custody MENA FZE")]
    BitGoCustodyMENAFZE,
    #[serde(rename = "BitGo Sister Trust 1")]
    BitGoSisterTrust1,
}

/// User data for wallet
#[apply(Response)]
pub struct WalletUser {
    /// User ID (BitGo internal unique ID)
    pub user: Option<String>,
    /// User permissions - required
    /// Allowed values: admin, spend, view, freeze, trade
    /// - admin: add/remove wallet policies, add/remove wallet users, approve/reject wallet pending approvals, withdraw assets from wallet, generate new receive addresses, view all balances and transactions
    /// - spend: withdraw assets from wallet, generate new receive addresses, view all balances and transactions
    /// - view: generate new receive addresses, view all balances and transactions
    /// - freeze
    /// - trade: trade on a wallet - applicable only for trading wallets
    pub permissions: Vec<String>,
    /// Whether the user needs recovery
    pub needs_recovery: Option<bool>,
    /// Whether the user is recoverable
    pub recoverable: Option<bool>,
}

/// Key signature data
#[apply(Response)]
pub struct KeySignatures {
    /// Backup key signature
    pub backup: Option<String>,
    /// BitGo key signature
    pub bitgo: Option<String>,
}

/// Freeze data
#[apply(Response)]
pub struct FreezeData {
    /// Time of freeze expiration
    pub expires: Option<String>,
    /// Time of freeze creation
    pub time: Option<String>,
}

/// Admin settings for wallet
#[apply(Response)]
pub struct WalletAdmin {
    /// Policy settings
    pub policy: Option<serde_json::Value>,
}

/// Build defaults
#[apply(Response)]
pub struct BuildDefaults {
    /// Default fee type
    pub fee_type: Option<String>,
    /// Default fee rate
    pub fee_rate: Option<u64>,
}

/// Receive address for wallet
#[apply(Response)]
pub struct ReceiveAddress {
    /// Address string (e.g., "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS")
    /// Max length: 250 characters
    pub address: String,

    /// Address type
    pub address_type: Option<String>,

    /// Coin - required
    /// A cryptocurrency symbol or token ticker symbol (e.g., "btc")
    pub coin: String,

    /// Wallet ID - required
    pub wallet: String,

    /// Chain number - required
    /// Allowed values: 0, 1, 10, 11, 20, 21, 30, 31, 40, 41
    pub chain: u32,

    /// Index number - required
    pub index: u32,

    /// Platform public ID for an address
    /// Example: "59cd72485007a239fb00282ed480da1f"
    /// Pattern: ^[0-9a-f]{32}$
    pub id: Option<String>,

    /// Balance object
    pub balance: Option<serde_json::Value>,

    /// Properties specific to certain coin types
    pub coin_specific: Option<serde_json::Value>,

    /// A human-readable label for the address
    /// Max length: 250 characters
    pub label: Option<String>,

    /// Last nonce (default: -1)
    pub last_nonce: Option<i64>,

    /// Token
    pub token: Option<String>,

    /// Proof
    pub proof: Option<String>,

    /// Signature
    pub signature: Option<String>,

    /// Last consolidated time (date string)
    pub last_consolidated_time: Option<String>,

    /// Whether consolidation is needed
    pub needs_consolidation: Option<bool>,

    /// Token consolidation state
    pub token_consolidation_state: Option<String>,

    /// Token address
    pub token_address: Option<String>,

    /// Creation timestamp
    pub creation_date: Option<String>,
}

/// Client flags for wallet
#[apply(Response)]
pub struct ClientFlag {
    /// Key
    pub name: String,
    /// Value
    pub value: serde_json::Value,
}

/// Wallet config
#[apply(Response)]
pub struct WalletConfig {
    /// Fallback factorA
    pub fallback_factor_a: Option<String>,
    /// Whether MPC is enabled
    pub mpc_enabled: Option<bool>,
    /// Whether hardware is enabled
    pub hardware_enabled: Option<bool>,
}

/// Wallet data returned from the API
#[serde_as]
#[apply(Response)]
pub struct Wallet {
    /// Whether backup key signing is allowed
    pub allow_backup_key_signing: bool,
    /// Number of approvals required
    pub approvals_required: u32,
    /// Coin symbol
    #[serde_as(as = "OneOrMany<_>")]
    pub coin: Vec<Coin>,
    /// Coin-specific data
    pub coin_specific: serde_json::Value,
    /// Whether the wallet is deleted
    pub deleted: bool,
    /// Whether transaction notifications are disabled
    pub disable_transaction_notifications: bool,
    /// Whether the wallet has a large number of addresses
    pub has_large_number_of_addresses: bool,
    /// Wallet ID
    pub id: String,
    /// Whether the wallet is cold
    pub is_cold: bool,
    /// Wallet label
    pub label: String,
    /// Wallet creation time
    pub start_date: String,

    // Optional fields
    /// Admin settings
    pub admin: Option<WalletAdmin>,
    /// Billing enterprise
    pub billing_enterprise: Option<String>,
    /// Build defaults
    pub build_defaults: Option<BuildDefaults>,
    /// Client flags
    pub client_flags: Option<Vec<ClientFlag>>,
    /// Wallet config
    pub config: Option<WalletConfig>,
    /// Custodial wallet ID
    pub custodial_wallet_id: Option<String>,
    /// Custom change key signatures
    pub custom_change_key_signatures: Option<HashMap<String, String>>,
    /// Customer wallet ID
    pub customer_wallet_id: Option<String>,
    /// Enterprise ID
    pub enterprise: Option<String>,
    /// Organization
    pub organization: Option<String>,
    /// BitGo organization
    pub bitgo_org: Option<String>,
    /// Freeze data
    pub freeze: Option<FreezeData>,
    /// Instant provider
    pub instant_provider: Option<String>,
    /// Keys
    pub keys: Option<Vec<String>>,
    /// Key signatures
    pub key_signatures: Option<KeySignatures>,
    /// Number of signatures required
    pub m: Option<u32>,
    /// Migrated from
    pub migrated_from: Option<String>,
    /// Multisig type (onchain, tss, blsdkg)
    pub multisig_type: Option<String>,
    /// Multisig type version
    pub multisig_type_version: Option<String>,
    /// Number of keys provided
    pub n: Option<u32>,
    /// Whether the wallet is recoverable
    pub recoverable: Option<bool>,
    /// Tags
    pub tags: Option<Vec<String>>,
    /// Wallet type
    #[serde(rename = "type")]
    pub wallet_type: Option<String>,
    /// Wallet sub-type
    pub sub_type: Option<String>,

    // Balance fields (only returned if expandBalance=true)
    /// Balance in string representation
    #[serde(rename = "balanceString")]
    pub balance: Option<BaseAmount>,
    /// Confirmed balance in string representation
    #[serde(rename = "confirmedBalanceString")]
    pub confirmed_balance: Option<BaseAmount>,
    /// Spendable balance in string representation
    #[serde(rename = "spendableBalanceString")]
    pub spendable_balance: Option<BaseAmount>,
    /// Staking balance in string representation (only if includeStakingBalances=true)
    #[serde(rename = "stakingBalanceString")]
    pub staking_balance: Option<BaseAmount>,
    /// Reward balance in string representation (only if includeStakingBalances=true)
    #[serde(rename = "rewardBalanceString")]
    pub reward_balance: Option<BaseAmount>,

    /// Users with access to the wallet
    pub users: Option<Vec<WalletUser>>,
    /// Wallet flags
    pub wallet_flags: Option<Vec<ClientFlag>>,
    /// Receive address
    pub receive_address: Option<ReceiveAddress>,
}

impl Response for Wallet {}
