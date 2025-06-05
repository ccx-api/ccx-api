//! BitGo Get Transfer API implementation
//!
//! This module implements the BitGo "Get transfer" API endpoint which allows
//! retrieving information about a single transfer by ID. A transfer is a
//! wallet-specific object that represents deposits or withdrawals.
//!
//! # API Documentation
//! Based on: https://developers.bitgo.com/api/v2.wallet.gettransfer
//!
//! # Supported Features
//! - Fetch single transfer by ID
//! - Optional include transaction request created date
//! - Returns complete transfer information including entries, history, and coin-specific data
//!
//! # Usage
//! ```rust,no_run
//! use ccx_bitgo::prelude::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let result = transfer::GetTransfer::builder()
//!     .coin("btc")
//!     .wallet_id("59cd72485007a239fb00282ed480da1f")
//!     .transfer_id("transfer_id_123")
//!     .include_tx_request_created_date(true)
//!     .build();
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::rate_limits::RateLimitType;

// Re-export shared types from types::transfer module
use crate::types::transfer::Transfer;

/// Request to get a single transfer by ID
#[apply(Request)]
pub struct GetTransfer {
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    wallet_id: String,

    #[serde(skip)]
    transfer_id: String,

    /// Include transaction request created date in response
    include_tx_request_created_date: Option<bool>,
}

impl Response for Transfer {}

impl Request for GetTransfer {
    type Response = Transfer;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        let coin = &self.coin;
        let wallet_id = &self.wallet_id;
        let transfer_id = &self.transfer_id;

        format!("/api/v2/{coin}/wallet/{wallet_id}/transfer/{transfer_id}").into()
    }
}

impl SignedRequest for GetTransfer {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    mod request_serialization {
        use super::*;

        #[test]
        fn test_basic_request_no_parameters() {
            let request = GetTransfer::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .transfer_id("transfer_id_123")
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            // Should be empty for requests with no query parameters
            assert_eq!(parsed, serde_json::json!({}));
        }

        #[test]
        fn test_request_with_include_tx_request_created_date() {
            let request = GetTransfer::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .transfer_id("transfer_id_123")
                .include_tx_request_created_date(true)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "includeTxRequestCreatedDate": true
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
                "replacedBy": [],
                "inputs": [
                    {
                        "id": "003f688cc349f1fca8ac5ffa21671ca911b6ef351085c60733ed8c2ebf162cb8:2",
                        "address": "2MsKxhhkDo5WaLaYRGA9Cr3iSQPyXsu6Fi2",
                        "value": 0,
                        "valueString": "2000000",
                        "blockHeight": 0,
                        "date": "2017-03-25T23:01:40.248Z",
                        "coinbase": true,
                        "wallet": "59cd72485007a239fb00282ed480da1f",
                        "fromWallet": "59cd72485007a239fb00282ed480da1f",
                        "chain": 0,
                        "index": 0,
                        "redeemScript": "522102f1e990044d2a8be43d5b500bbdcb36277b97a4b07e01c5101ae8ec1568bfd6532103dab7dc82f2fc8c28200c1bdeca9c4cf181e0ca257395829cbd599395048afb57210205422e711827d8356f2fb75334d863941dd7eb45bd5788fa231dc5fa755135b653ae",
                        "witnessScript": "52210351311cd81144e6cbdba561d24dfc22644cb02d053339d4beace03231b3be4f372103a8d0c1a375b9ee1a2411f9f8e18373be7f228b18260f63bbfca48809170ed08b2103c3bd8bd074657bbe9ee6714b31a4a54b6fd5b5cda0e1030122f9bf46b5034f6b53ae",
                        "isSegwit": true
                    }
                ],
                "outputs": [
                    {
                        "id": "003f688cc349f1fca8ac5ffa21671ca911b6ef351085c60733ed8c2ebf162cb8:2",
                        "address": "2MsKxhhkDo5WaLaYRGA9Cr3iSQPyXsu6Fi2",
                        "value": 0,
                        "valueString": "2000000",
                        "blockHeight": 0,
                        "date": "2017-03-25T23:01:40.248Z",
                        "coinbase": true,
                        "wallet": "59cd72485007a239fb00282ed480da1f",
                        "fromWallet": "59cd72485007a239fb00282ed480da1f",
                        "chain": 0,
                        "index": 0,
                        "redeemScript": "522102f1e990044d2a8be43d5b500bbdcb36277b97a4b07e01c5101ae8ec1568bfd6532103dab7dc82f2fc8c28200c1bdeca9c4cf181e0ca257395829cbd599395048afb57210205422e711827d8356f2fb75334d863941dd7eb45bd5788fa231dc5fa755135b653ae",
                        "witnessScript": "52210351311cd81144e6cbdba561d24dfc22644cb02d053339d4beace03231b3be4f372103a8d0c1a375b9ee1a2411f9f8e18373be7f228b18260f63bbfca48809170ed08b2103c3bd8bd074657bbe9ee6714b31a4a54b6fd5b5cda0e1030122f9bf46b5034f6b53ae",
                        "isSegwit": true
                    }
                ]
            }"#;

            let response: Transfer =
                serde_json::from_str(response_json).expect("Failed to deserialize response");

            assert_eq!(response.coin.to_string(), "btc");
            assert_eq!(response.id, "59cd72485007a239fb00282ed480da1f");
            assert_eq!(
                response.state,
                crate::types::transfer::TransferState::Confirmed
            );
            // Compare serialized string representation since JSON contains negative value
            assert_eq!(response.value.to_string(), "2000000");
            assert_eq!(response.comment, Some("Test transfer".to_string()));
            assert_eq!(response.sequence_id, Some("test_sequence_123".to_string()));

            // Check entries
            assert!(response.entries.is_some());
            let entries = response.entries.as_ref().unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].address, "2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ");
            assert_eq!(entries[0].value.to_string(), "2000000");

            // Check history
            assert_eq!(response.history.len(), 1);
            assert_eq!(response.history[0].action, "created");
            assert_eq!(
                response.history[0].comment,
                Some("Test transfer".to_string())
            );

            // Check inputs and outputs
            assert!(response.inputs.is_some());
            assert!(response.outputs.is_some());
            let inputs = response.inputs.as_ref().unwrap();
            let outputs = response.outputs.as_ref().unwrap();
            assert_eq!(inputs.len(), 1);
            assert_eq!(outputs.len(), 1);
        }
    }

    mod path_generation {
        use super::*;

        #[test]
        fn test_path_construction() {
            let request = GetTransfer::builder()
                .coin("btc")
                .wallet_id("wallet123")
                .transfer_id("transfer456")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/btc/wallet/wallet123/transfer/transfer456");
        }

        #[test]
        fn test_path_construction_with_token() {
            let request = GetTransfer::builder()
                .coin("tsol:usdcv2")
                .wallet_id("wallet456")
                .transfer_id("transfer789")
                .build();

            let path = request.path();
            assert_eq!(
                path,
                "/api/v2/tsol:usdcv2/wallet/wallet456/transfer/transfer789"
            );
        }
    }
}
