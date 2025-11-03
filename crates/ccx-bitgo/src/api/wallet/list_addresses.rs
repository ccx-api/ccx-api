use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;
use crate::types::wallet::ReceiveAddress;

/// Request to list addresses on a wallet
#[apply(Request)]
pub struct ListAddresses {
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    wallet_id: String,

    /// Whether to include address balances (default: false)
    include_balances: Option<bool>,

    /// Whether to include token addresses (default: false)
    include_tokens: Option<bool>,

    /// Whether to include all token addresses for receiver addresses (default: false)
    include_all_token_addresses: Option<bool>,

    /// A case-insensitive regular expression which will be used to filter returned addresses based on their address label
    label_contains: Option<String>,

    /// Maximum number of results to return (1 to 500, default: 25)
    limit: Option<u32>,

    /// Sort order of returned addresses (1 for ascending, -1 for descending, default: 1)
    sort: Option<i32>,

    /// The field by which addresses will be sorted (token or label, default: sort by id)
    sorted_field: Option<SortedField>,

    /// Sort order of the returned addresses on the sortedField (1 for ascending, -1 for descending)
    sorted_field_direction: Option<i32>,

    /// Filter by address chains
    chains: Option<Vec<u32>>,

    /// For large wallets (>100k addresses), include total count of addresses (including addresses pending on-chain) matching the query (default: false)
    include_total_address_count: Option<bool>,

    /// Return the next batch of results, based on the nextBatchPrevId value from the previous batch
    prev_id: Option<String>,

    /// This param is used to query and filter addresses by token names in case of ofc wallets
    token: Option<Vec<String>>,

    /// Name of the token that the response should include balances for (Eth and Celo only)
    return_balances_for_token: Option<String>,

    /// Filter the addresses based on their deployment status (Eth only).
    /// Return the deployed addresses if this param is passed as false and return undeployed addresses if it is passed as true
    pending_deployment: Option<bool>,

    /// DEPRECATED. Mutually exclusive with 'chains'. Returns only 'p2shP2wsh' unspents/addresses on true.
    /// Returns only 'p2sh' unspents/addresses on false.
    /// Equivalent to passing 10 and 11 as the only value in 'chains' on true, 0 and 1 on false.
    segwit: Option<bool>,
}

/// The field by which addresses will be sorted
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortedField {
    /// Sort by token
    Token,
    /// Sort by label
    Label,
}

/// Response for ListAddresses request
#[apply(Response)]
pub struct ListAddressesResponse {
    /// Coin symbol
    pub coin: String,
    /// Total number of addresses pending on-chain initialization on this wallet
    /// Note: for wallets with many addresses (100,000 or more), this property may be omitted for performance reasons
    pub pending_address_count: Option<u32>,
    /// Total number of addresses which match the provided query parameters
    /// Note: for wallets with many addresses (100,000 or more), this property may be omitted for performance reasons
    pub total_address_count: Option<u32>,
    /// List of addresses
    pub addresses: Vec<ReceiveAddress>,
    /// ID to use for fetching the next batch of results
    pub next_batch_prev_id: Option<String>,
    /// Total number of addresses returned in this response
    pub count: Option<u32>,
}

impl Response for ListAddressesResponse {}

impl Request for ListAddresses {
    type Response = ListAddressesResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        let coin = &self.coin;
        let wallet_id = &self.wallet_id;

        format!("/api/v2/{coin}/wallet/{wallet_id}/addresses").into()
    }
}

impl SignedRequest for ListAddresses {}

