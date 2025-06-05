//! BitGo List Transfers API implementation
//!
//! This module implements the BitGo "List transfers" API endpoint which allows
//! retrieving deposits and withdrawals for a wallet. Transfers are sorted in
//! descending order by height, then ID.
//!
//! # API Documentation
//! Based on: https://developers.bitgo.com/api/v2.wallet.listtransfers
//!
//! # Supported Features
//! - Pagination with limit and prevId
//! - Filtering by date range, height, state, type, value range
//! - Sorting and reverse order
//! - Address and memo filtering
//! - Include additional data (hex, RBF transfers)
//!
//! # Usage
//! ```rust,no_run
//! use ccx_bitgo::prelude::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let result = transfer::ListTransfers::builder()
//!     .coin("btc")
//!     .wallet_id("wallet_id")
//!     .limit(10u32)
//!     .state(wallet::TransferState::Confirmed)
//!     .build();
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;

// Re-export shared types from types::transfer module
use crate::types::transfer::{Transfer, TransferSortBy, TransferState, TransferType};

/// Request to list transfers from a wallet
#[apply(Request)]
pub struct ListTransfers {
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    wallet_id: String,

    /// Include data for all subtokens (ERC20, Stellar tokens)
    all_tokens: Option<bool>,

    /// Return transfers with date >= this timestamp
    date_gte: Option<String>,

    /// Return transfers with date < this timestamp
    date_lt: Option<String>,

    /// Block or ledger height
    height: Option<String>,

    /// Maximum number of results (1-500, default 25)
    limit: Option<u32>,

    /// Return next batch based on previous batch's nextBatchPrevId
    prev_id: Option<String>,

    /// Filter by transfer states
    state: Option<TransferState>,

    /// Filter by transfer type (send/receive)
    #[serde(rename = "type")]
    transfer_type: Option<TransferType>,

    /// Return transfers with value >= this amount
    value_gte: Option<i64>,

    /// Return transfers with value < this amount
    value_lt: Option<i64>,

    /// Sort key for transfers
    sort_by: Option<TransferSortBy>,

    /// Return results in reverse order
    reverse: Option<bool>,

    /// Filter by specific transfer ID
    id: Option<String>,

    /// Filter by pending approval ID
    pending_approval_id: Option<String>,

    /// Filter by addresses
    address: Option<Vec<String>>,

    /// Include raw hex data in response
    include_hex: Option<bool>,

    /// Filter by memo IDs (Stellar, EOS)
    memo_id: Option<Vec<String>>,

    /// Include Replace-By-Fee (RBF) transfers
    include_rbf: Option<bool>,
}

/// Response for ListTransfers request
#[apply(Response)]
pub struct ListTransfersResponse {
    /// List of transfers
    pub transfers: Vec<Transfer>,

    /// Coin identifier
    pub coin: Coin,

    /// Next batch ID for pagination
    pub next_batch_prev_id: Option<String>,
}

impl Response for ListTransfersResponse {}

impl Request for ListTransfers {
    type Response = ListTransfersResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        let coin = &self.coin;
        let wallet_id = &self.wallet_id;

        format!("/api/v2/{coin}/wallet/{wallet_id}/transfer").into()
    }
}

impl SignedRequest for ListTransfers {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    mod request_serialization {
        use super::*;

        #[test]
        fn test_basic_request_no_parameters() {
            let request = ListTransfers::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            // Should be empty for requests with no query parameters
            assert_eq!(parsed, serde_json::json!({}));
        }

        #[test]
        fn test_request_with_parameters() {
            let request = ListTransfers::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .limit(10u32)
                .state(TransferState::Confirmed)
                .transfer_type(TransferType::Send)
                .reverse(true)
                .include_hex(false)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "limit": 10,
                "state": "confirmed",
                "type": "send",
                "reverse": true,
                "includeHex": false
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_request_with_date_and_value_filters() {
            let request = ListTransfers::builder()
                .coin("eth")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .date_gte("2025-01-01T00:00:00Z")
                .date_lt("2025-12-31T23:59:59Z")
                .value_gte(1000000)
                .value_lt(10000000)
                .sort_by(TransferSortBy::HeightId)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "dateGte": "2025-01-01T00:00:00Z",
                "dateLt": "2025-12-31T23:59:59Z",
                "valueGte": 1000000,
                "valueLt": 10000000,
                "sortBy": "heightId"
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_request_with_address_filtering() {
            let request = ListTransfers::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .address(vec![
                    "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS".to_string(),
                    "2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ".to_string(),
                ])
                .all_tokens(true)
                .include_rbf(true)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "address": [
                    "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS",
                    "2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ"
                ],
                "allTokens": true,
                "includeRbf": true
            });

            assert_eq!(parsed, expected);
        }
    }

    mod response_deserialization {
        use super::*;

        #[test]
        fn test_successful_response() {
            // Use actual response format from BitGo API documentation
            let response_json = r#"{
                "transfers": [
                    {
                        "coin": "btc",
                        "id": "59cd72485007a239fb00282ed480da1f",
                        "wallet": "59cd72485007a239fb00282ed480da1f",
                        "enterprise": "59cd72485007a239fb00282ed480da1f",
                        "txid": "b8a828b98dbf32d9fd1875cbace9640ceb8c82626716b4a64203fdc79bb46d26",
                        "txidType": "transactionHash",
                        "height": 650000,
                        "heightId": "650000-59cd72485007a239fb00282ed480da1f",
                        "date": "2019-08-24T14:15:22Z",
                        "confirmations": 6,
                        "type": "send",
                        "value": -2000000,
                        "valueString": "2000000",
                        "intendedValueString": "2000000",
                        "baseValue": 1995000,
                        "baseValueString": "1995000",
                        "baseValueWithoutFees": 2000000,
                        "baseValueWithoutFeesString": "2000000",
                        "feeString": "5000",
                        "payGoFee": 0,
                        "payGoFeeString": "0",
                        "usd": -100.0,
                        "usdRate": 50000.0,
                        "state": "confirmed",
                        "tags": ["59cd72485007a239fb00282ed480da1f"],
                        "history": [
                            {
                                "date": "2019-08-24T14:15:22Z",
                                "user": "59cd72485007a239fb00282ed480da1f",
                                "action": "created",
                                "comment": "Test transfer"
                            }
                        ],
                        "comment": "Test transfer",
                        "vSize": 250,
                        "coinSpecific": {},
                        "sequenceId": "test_sequence_123",
                        "entries": [
                            {
                                "address": "2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ",
                                "wallet": "59cd72485007a239fb00282ed480da1f",
                                "value": -2000000,
                                "valueString": "2000000",
                                "isChange": false,
                                "isPayGo": false,
                                "token": null,
                                "label": "Test address",
                                "failed": false
                            }
                        ],
                        "usersNotified": true,
                        "label": "Test address",
                        "replaces": [],
                        "replacedBy": []
                    }
                ],
                "coin": "btc",
                "nextBatchPrevId": "585951a5df8380e0e3063e9f"
            }"#;

            let response: ListTransfersResponse =
                serde_json::from_str(response_json).expect("Failed to deserialize response");

            assert_eq!(response.coin.to_string(), "btc");
            assert_eq!(response.transfers.len(), 1);
            assert_eq!(
                response.next_batch_prev_id,
                Some("585951a5df8380e0e3063e9f".to_string())
            );

            let transfer = &response.transfers[0];
            assert_eq!(transfer.id, "59cd72485007a239fb00282ed480da1f");
            assert_eq!(transfer.coin.to_string(), "btc");
            assert_eq!(
                transfer.state,
                crate::types::transfer::TransferState::Confirmed
            );
            assert_eq!(transfer.value.to_string(), "2000000");
            assert_eq!(transfer.comment, Some("Test transfer".to_string()));
            assert_eq!(transfer.sequence_id, Some("test_sequence_123".to_string()));

            // Check entries
            assert!(transfer.entries.is_some());
            let entries = transfer.entries.as_ref().unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].address, "2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ");
            assert_eq!(entries[0].value.to_string(), "2000000");

            // Check history
            assert_eq!(transfer.history.len(), 1);
            assert_eq!(transfer.history[0].action, "created");
            assert_eq!(
                transfer.history[0].comment,
                Some("Test transfer".to_string())
            );
        }

        #[test]
        fn test_empty_response() {
            let response_json = r#"{
                "transfers": [],
                "coin": "btc",
                "nextBatchPrevId": null
            }"#;

            let response: ListTransfersResponse =
                serde_json::from_str(response_json).expect("Failed to deserialize empty response");

            assert_eq!(response.coin.to_string(), "btc");
            assert_eq!(response.transfers.len(), 0);
            assert_eq!(response.next_batch_prev_id, None);
        }
    }

    mod path_generation {
        use super::*;

        #[test]
        fn test_path_construction() {
            let request = ListTransfers::builder()
                .coin("btc")
                .wallet_id("wallet123")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/btc/wallet/wallet123/transfer");
        }

        #[test]
        fn test_path_construction_with_token() {
            let request = ListTransfers::builder()
                .coin("tsol:usdcv2")
                .wallet_id("wallet456")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/tsol:usdcv2/wallet/wallet456/transfer");
        }
    }
}
