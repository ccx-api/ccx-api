use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;
use crate::types::wallet::*;

/// Request to list wallets
#[apply(Request)]
pub struct ListWallets {
    /// Filter by wallet ID
    id: Option<Vec<String>>,

    /// Filter by coin
    coin: Option<Vec<Coin>>,

    /// Filter by enterprise ID
    enterprise: Option<Vec<String>>,

    /// Filter by wallet type
    #[serde(rename = "type")]
    wallet_type: Option<Vec<WalletType>>,

    /// Filter by wallet sub-type
    sub_type: Option<Vec<WalletSubType>>,

    /// Filter by deleted state
    deleted: Option<bool>,

    /// Return the next batch of results, based on the "nextBatchPrevId" value from the previous batch
    prev_id: Option<String>,

    /// Filter by label substring
    label_contains: Option<String>,

    /// Add "balanceString", "confirmedBalanceString" and "spendableBalanceString" to each wallet
    expand_balance: Option<bool>,

    /// Whether to exclude spendable balance
    exclude_spendable_balance: Option<bool>,

    /// Whether to expand policy
    expand_policy: Option<bool>,

    /// Whether linked custodial wallets should be expanded inline
    expand_custodial_wallet: Option<bool>,

    /// Whether to ignore errors
    ignore_errors: Option<bool>,

    /// Include `stakingBalanceString` and `rewardBalanceString` properties for each staking wallet.
    /// Requires `expandBalance` to be set to true.
    include_staking_balances: Option<bool>,

    /// Maximum number of results to return. If the result set is truncated,
    /// use the "nextBatchPrevId" value to get the next batch.
    limit: Option<u32>,

    /// Number of documents to skip for offset-based pagination
    offset: Option<u32>,

    /// Do not add "receiveAddress" to each wallet
    skip_receive_address: Option<bool>,

    /// Return only wallets for which the user has the given permission
    permission: Option<WalletPermission>,

    /// Return only pinned wallets ("pinnedOnly") or only unpinned wallets ("pinnedExcluded")
    pinned_wallets: Option<PinnedWallets>,

    /// Return only wallets belong to the BitGo trust org
    bitgo_org: Option<BitGoOrg>,
}

/// Response for ListWallets request
#[apply(Response)]
pub struct ListWalletsResponse {
    /// List of wallets
    pub wallets: Vec<Wallet>,
    /// ID to use for fetching the next batch of results
    pub next_batch_prev_id: Option<String>,
    /// Total count of wallets
    pub total_count: Option<u32>,
}

impl Response for ListWalletsResponse {}

impl Request for ListWallets {
    type Response = ListWalletsResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "/api/v2/wallets".into()
    }
}

impl SignedRequest for ListWallets {}
