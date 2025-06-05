//! BitGo Send Many API implementation
//!
//! This module implements the BitGo "Send to many" API endpoint which allows
//! sending coins or tokens to multiple recipients in a single transaction.
//! This is useful for bulk transactions and helps reduce aggregate blockchain fees.
//!
//! # API Documentation
//! Based on: https://developers.bitgo.com/api/express.wallet.sendmany
//!
//! # Supported Features
//! - Send to multiple recipients in one transaction
//! - Support for both coins and tokens
//! - Custom fee settings (fee rate, gas price, etc.)
//! - Transaction metadata (memos, comments, sequence IDs)
//! - Advanced options (change addresses, unspent selection, etc.)
//!
//! # Usage
//! ```rust,no_run
//! use ccx_bitgo::prelude::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let recipient1 = wallet::Recipient::builder()
//!     .address("2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS")
//!     .amount(1000000i128)
//!     .build();
//!
//! let recipient2 = wallet::Recipient::builder()
//!     .address("2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ")
//!     .amount(2000000i128)
//!     .build();
//!
//! let result = wallet::SendMany::builder()
//!     .coin("btc")
//!     .wallet_id("wallet_id")
//!     .recipients(vec![recipient1, recipient2])
//!     .wallet_passphrase("passphrase")
//!     .tx_type(wallet::TransactionType::Transfer)
//!     .comment("Multi-recipient transaction")
//!     .build();
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, SignedRequest};
use crate::types::BaseAmount;
use crate::types::derive::Request;
use crate::types::rate_limits::RateLimitType;

// Re-export shared types from send_coins.rs
use super::send_coins::{
    Eip1559Settings, ReservationSettings, SendCoinsResponse, StakingOptions, TransactionMemo,
    TransactionType, Trustline,
};

/// Token data for token transactions
#[apply(Request)]
pub struct TokenData {
    /// Token name
    token_name: Option<String>,

    /// Token contract address
    token_contract_address: Option<String>,

    /// Number of decimal places
    decimal_places: Option<u32>,

    /// Token type (e.g., ERC20)
    token_type: Option<String>,

    /// Token ID
    token_id: Option<String>,

    /// Token quantity
    token_quantity: Option<String>,
}

/// Recipient for send many operation
#[apply(Request)]
pub struct Recipient {
    /// Destination address
    address: String,

    /// Amount in base units (e.g. satoshis) to send
    amount: BaseAmount,

    /// Token name (required for MPC wallets token transactions)
    token_name: Option<String>,

    /// Data for token interaction
    token_data: Option<TokenData>,
}

/// Request to send coins to multiple recipients
#[apply(Request)]
pub struct SendMany {
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    wallet_id: String,

    /// List of recipient addresses and amounts to send
    recipients: Vec<Recipient>,

    /// Two factor auth code
    otp: Option<String>,

    /// Wallet passphrase to decrypt the user key
    wallet_passphrase: Option<String>,

    /// Private key in string form (optional, if wallet_passphrase is not available)
    prv: Option<String>,

    /// Transaction type (required for MPC wallets)
    #[serde(rename = "type")]
    tx_type: TransactionType,

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

    /// Address type for change
    change_address_type: Option<String>,

    /// Start time for transaction validity window (HBAR only)
    start_time: Option<f64>,

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
}

/// Response for SendMany request (reuses SendCoinsResponse structure)
pub type SendManyResponse = SendCoinsResponse;

impl Request for SendMany {
    type Response = SendManyResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        let coin = &self.coin;
        let wallet_id = &self.wallet_id;

        format!("/api/v2/{coin}/wallet/{wallet_id}/sendmany").into()
    }
}

impl SignedRequest for SendMany {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    mod request_serialization {
        use super::*;

        #[test]
        fn test_basic_request_with_recipients() {
            let recipient = Recipient::builder()
                .address("2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS")
                .amount(2000000i128)
                .build();

            let request = SendMany::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .recipients(vec![recipient])
                .tx_type(TransactionType::Transfer)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "recipients": [
                    {
                        "address": "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS",
                        "amount": "2000000"
                    }
                ],
                "type": "transfer"
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_request_with_token_recipient() {
            let token_data = TokenData::builder()
                .token_name("sol:natix")
                .token_contract_address("contract123")
                .decimal_places(8u32)
                .token_type("ERC20")
                .build();

            let recipient = Recipient::builder()
                .address("2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS")
                .amount(2000000i128)
                .token_name("sol:natix")
                .token_data(token_data)
                .build();

            let request = SendMany::builder()
                .coin("tsol")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .recipients(vec![recipient])
                .wallet_passphrase("test_passphrase")
                .tx_type(TransactionType::Transfer)
                .comment("Test multi-recipient transaction")
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "recipients": [
                    {
                        "address": "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS",
                        "amount": "2000000",
                        "tokenName": "sol:natix",
                        "tokenData": {
                            "tokenName": "sol:natix",
                            "tokenContractAddress": "contract123",
                            "decimalPlaces": 8,
                            "tokenType": "ERC20"
                        }
                    }
                ],
                "walletPassphrase": "test_passphrase",
                "type": "transfer",
                "comment": "Test multi-recipient transaction"
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_request_with_multiple_recipients() {
            let recipient1 = Recipient::builder()
                .address("2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS")
                .amount(1000000i128)
                .build();

            let recipient2 = Recipient::builder()
                .address("2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ")
                .amount(2000000i128)
                .build();

            let request = SendMany::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .recipients(vec![recipient1, recipient2])
                .tx_type(TransactionType::Transfer)
                .fee_rate(5000i128)
                .sequence_id("unique_sequence_123")
                .comment("Multi-recipient test")
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "recipients": [
                    {
                        "address": "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS",
                        "amount": "1000000"
                    },
                    {
                        "address": "2NAUwNgXaoFj2VVnSEvNLGuez8CfdU2UCMZ",
                        "amount": "2000000"
                    }
                ],
                "type": "transfer",
                "feeRate": "5000",
                "sequenceId": "unique_sequence_123",
                "comment": "Multi-recipient test"
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_request_with_advanced_parameters() {
            let eip1559 = Eip1559Settings::builder()
                .max_priority_fee_per_gas(2000000i128)
                .max_fee_per_gas(4000000i128)
                .build();

            let memo = TransactionMemo::builder()
                .memo_type("text")
                .value("Transaction memo")
                .build();

            let recipient = Recipient::builder()
                .address("2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS")
                .amount(2000000i128)
                .build();

            let request = SendMany::builder()
                .coin("eth")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .recipients(vec![recipient])
                .wallet_passphrase("test_passphrase")
                .tx_type(TransactionType::Transfer)
                .gas_limit("21000")
                .eip1559(eip1559)
                .memo(memo)
                .no_split_change(true)
                .enforce_min_confirms_for_change(true)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse");

            // Check some key fields are present
            assert!(parsed["recipients"].is_array());
            assert_eq!(parsed["recipients"].as_array().unwrap().len(), 1);
            assert_eq!(parsed["walletPassphrase"], "test_passphrase");
            assert_eq!(parsed["gasLimit"], "21000");
            assert_eq!(parsed["noSplitChange"], true);
            assert_eq!(parsed["enforceMinConfirmsForChange"], true);
            assert!(parsed["eip1559"].is_object());
            assert!(parsed["memo"].is_object());
        }
    }

    mod response_deserialization {
        use super::*;

        #[test]
        fn test_successful_response() {
            // Use actual response format from real BitGo API call
            let response_json = r#"{
                "transfer": {
                    "entries": [
                        {
                            "address": "DXg3RSUZZ7fyKqFo4oxFmmFTRR1P6BifsVnxFANRwfx8",
                            "wallet": "6863d854f719728e7976d52e23822b68",
                            "value": -1000,
                            "valueString": "1000"
                        },
                        {
                            "address": "J2wPANkdB1Vs6iKgJz9h9ik9uZkxNWYGTvgyhPN3AV4u",
                            "value": 1000,
                            "valueString": "1000"
                        }
                    ],
                    "id": "687a1aa22292af2aed63473e8ae224dc",
                    "coin": "tsol:usdcv2",
                    "wallet": "6863d854f719728e7976d52e23822b68",
                    "walletType": "hot",
                    "enterprise": "68276a3f26cc38614e83cb3dcdccb1ff",
                    "organization": "68276a3f26cc38614e83cb5e7bbe5800",
                    "txid": "3YpRUVbVwLMTkQaKts52z9Mo3ymaURPkDKKmezXHsN1fqDUgkLgTGsLZ3npU1sENG9R4NMGRCa34obxYoRSo3wWM",
                    "txidType": "transactionHash",
                    "txRequestId": "82c9e57a-5eff-487d-990d-57b4fc7cb3c6",
                    "height": 999999999,
                    "heightId": "999999999-687a1aa22292af2aed63473e8ae224dc",
                    "date": "2025-07-18T09:57:55.192Z",
                    "type": "send",
                    "value": -1000,
                    "valueString": "1000",
                    "intendedValueString": "1000",
                    "baseValue": 2048280,
                    "baseValueString": "2048280",
                    "baseValueWithoutFees": 1000,
                    "baseValueWithoutFeesString": "1000",
                    "feeString": "2049280",
                    "payGoFee": 0,
                    "payGoFeeString": "0",
                    "state": "signed",
                    "instant": false,
                    "isReward": false,
                    "isUnlock": false,
                    "isFee": false,
                    "senderInformationVerified": false,
                    "tags": ["6863d854f719728e7976d52e23822b68", "68276a3f26cc38614e83cb3dcdccb1ff"],
                    "history": [
                        {
                            "date": "2025-07-18T09:57:55.560Z",
                            "user": "68276a3d26cc38614e83ca7e63612771",
                            "action": "commented",
                            "comment": "Multi-recipient test transaction"
                        },
                        {
                            "date": "2025-07-18T09:57:55.192Z",
                            "action": "signed"
                        },
                        {
                            "date": "2025-07-18T09:57:54.624Z",
                            "user": "68276a3d26cc38614e83ca7e63612771",
                            "action": "created",
                            "comment": "Multi-recipient test transaction"
                        }
                    ],
                    "signedDate": "2025-07-18T09:57:55.192Z",
                    "comment": "Multi-recipient test transaction",
                    "metadata": [],
                    "commentedTime": "2025-07-18T09:57:55.560Z",
                    "signedTime": "2025-07-18T09:57:55.192Z",
                    "createdTime": "2025-07-18T09:57:54.624Z"
                },
                "txid": "3YpRUVbVwLMTkQaKts52z9Mo3ymaURPkDKKmezXHsN1fqDUgkLgTGsLZ3npU1sENG9R4NMGRCa34obxYoRSo3wWM",
                "tx": "An946PP0wymSvkR8a4FeOqHzkw/FfNJGKE2rU/MSMd+hKrWY56EjvP38ceSdFoMS96aAT7dhY3leRMzVswPrLA6BqY4Zt944U5XpNR3M5surydOSFSRFoqicHBYPwlSBOJ/Szew+vUEJRkKud2Qownb+UgGZypApRuPPvLR+c4ULAgEHDLonz+FqibNPr0PKMMqbHjqPkCZBhIuvwCxU8HEsZNglHJYXIETxIXw3hOjwL0niyPw1kegSlKtUOU+dIv17eo843SVkKwDwmO8ZJZumZwdWuuV/gURGQ2uBXNp07lKO11gfGSkKOf9dH1gf83FE7Ne4kJo6OnQvshEBCM6x7vQjq9qcz4rdFEUhoxYIvXDD4scgs/ZIGkfrFGMKA8bdTaQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADtELLORIVfxOpM9ATQoLQMrX/7NAaLb8bd5BgjfAC6njJclj04kifG7PRApFI4NgwtaE5na/xCEBI572Nvp+Fn9Fb+jnzNeeH9BYw+n8Nr1gz7SblkkjS2OhCcHyOLL+gan1RcZLFaO4IqEX3PSl4jPA1wxRbIas0TYBi6pQAAABqfVFxksXFEhjMlMPUrxf1ja7gibof1E49vZigAAAAAG3fbh12Whk9nL4UbO63msHLSF7V9bN5E6jPWFfv8AqeDXhfBd5YPZt2OAr3fPPldCbNIiIbq74kPzlRbg5cMmAwUDAgkBBAQAAAAHBwAECAYFCwoACwQDBgQACgzoAwAAAAAAAAY=",
                "status": "signed"
            }"#;

            let response: SendManyResponse =
                serde_json::from_str(response_json).expect("Failed to deserialize response");

            assert_eq!(response.transfer.coin.to_string(), "tsol:usdcv2");
            assert_eq!(response.transfer.id, "687a1aa22292af2aed63473e8ae224dc");
            assert_eq!(response.transfer.value.to_string(), "1000");
            assert_eq!(
                response.transfer.state,
                crate::types::transfer::TransferState::Signed
            );
            assert_eq!(
                response.transfer.comment,
                Some("Multi-recipient test transaction".to_string())
            );
            assert_eq!(response.transfer.wallet_type, Some("hot".to_string()));
            assert_eq!(response.transfer.instant, Some(false));
            assert_eq!(response.transfer.is_reward, Some(false));
            assert_eq!(response.transfer.is_unlock, Some(false));
            assert_eq!(response.transfer.is_fee, Some(false));
            assert_eq!(response.transfer.sender_information_verified, Some(false));
            assert_eq!(response.txid, Some("3YpRUVbVwLMTkQaKts52z9Mo3ymaURPkDKKmezXHsN1fqDUgkLgTGsLZ3npU1sENG9R4NMGRCa34obxYoRSo3wWM".to_string()));

            // Check entries
            assert!(response.transfer.entries.is_some());
            let entries = response.transfer.entries.unwrap();
            assert_eq!(entries.len(), 2);
            assert_eq!(
                entries[0].address,
                "DXg3RSUZZ7fyKqFo4oxFmmFTRR1P6BifsVnxFANRwfx8"
            );
            assert_eq!(entries[0].value.to_string(), "1000");
            assert_eq!(
                entries[1].address,
                "J2wPANkdB1Vs6iKgJz9h9ik9uZkxNWYGTvgyhPN3AV4u"
            );
            assert_eq!(entries[1].value.to_string(), "1000");

            // Check history
            assert_eq!(response.transfer.history.len(), 3);
            assert_eq!(response.transfer.history[0].action, "commented");
            assert_eq!(response.transfer.history[1].action, "signed");
            assert_eq!(response.transfer.history[2].action, "created");
        }
    }

    mod path_generation {
        use super::*;

        #[test]
        fn test_path_construction() {
            let recipient = Recipient::builder()
                .address("test_address")
                .amount(1000000i128)
                .build();

            let request = SendMany::builder()
                .coin("btc")
                .wallet_id("wallet123")
                .recipients(vec![recipient])
                .tx_type(TransactionType::Transfer)
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/btc/wallet/wallet123/sendmany");
        }

        #[test]
        fn test_path_construction_with_token() {
            let recipient = Recipient::builder()
                .address("test_address")
                .amount(1000000i128)
                .build();

            let request = SendMany::builder()
                .coin("tsol:usdcv2")
                .wallet_id("wallet456")
                .recipients(vec![recipient])
                .tx_type(TransactionType::Transfer)
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/tsol:usdcv2/wallet/wallet456/sendmany");
        }
    }
}
