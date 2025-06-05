use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, SignedRequest};
use crate::types::derive::Request;
use crate::types::rate_limits::RateLimitType;
use crate::types::wallet::Wallet;

/// Request to get wallet information by coin and ID
#[apply(Request)]
pub struct GetWalletByIdCoin {
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    wallet_id: String,

    /// Include data for all subtokens (i.e. ERC20 Tokens, Stellar Tokens)
    all_tokens: Option<bool>,

    /// True, if including unspent count for UTXO-based coins.
    unspent_count: Option<bool>,

    /// True, if including Replace-By-Fee (RBF) transactions in the total balance amount.
    include_rbf: Option<bool>,

    /// True, if including the advanced whitelist wallet address in the response.
    /// The address is annotated as part of the whitelist entry metadata
    expand_advanced_whitelist: Option<bool>,

    /// Includes the staked balance and reward balance of the wallet
    include_staking_balances: Option<bool>,

    /// Omit trying to fetch the wallet balance
    include_balance: Option<bool>,
}

impl Request for GetWalletByIdCoin {
    type Response = Wallet;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        let coin = &self.coin;
        let wallet_id = &self.wallet_id;

        format!("/api/v2/{coin}/wallet/{wallet_id}").into()
    }
}

impl SignedRequest for GetWalletByIdCoin {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    mod request_serialization {
        use super::*;

        #[test]
        fn test_basic_request_no_parameters() {
            // Test basic request with no optional parameters
            let request = GetWalletByIdCoin::builder()
                .coin("btc")
                .wallet_id("test_wallet_id")
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

            // Should be empty object since all fields are None or skipped
            assert_eq!(parsed, serde_json::json!({}));
        }

        #[test]
        fn test_request_with_all_parameters() {
            // Test request with all optional parameters set
            let request = GetWalletByIdCoin::builder()
                .coin("btc")
                .wallet_id("test_wallet_id")
                .all_tokens(true)
                .unspent_count(true)
                .include_rbf(false)
                .expand_advanced_whitelist(true)
                .include_staking_balances(true)
                .include_balance(false)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

            let expected = serde_json::json!({
                "allTokens": true,
                "unspentCount": true,
                "includeRbf": false,
                "expandAdvancedWhitelist": true,
                "includeStakingBalances": true,
                "includeBalance": false
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_partial_parameters() {
            // Test request with only some parameters set
            let request = GetWalletByIdCoin::builder()
                .coin("eth")
                .wallet_id("partial_test")
                .all_tokens(true)
                .include_staking_balances(false)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

            let expected = serde_json::json!({
                "allTokens": true,
                "includeStakingBalances": false
            });

            assert_eq!(parsed, expected);
        }
    }

    mod response_deserialization {
        use super::*;
        use crate::types::BaseAmount;

        #[test]
        fn test_full_response_with_balances() {
            // Sample JSON response based on BitGo API documentation
            let response_json = r#"{
                "allowBackupKeySigning": false,
                "approvalsRequired": 1,
                "coin": ["btc"],
                "coinSpecific": {},
                "deleted": false,

                "disableTransactionNotifications": false,
                "hasLargeNumberOfAddresses": false,
                "id": "59cd72485007a239fb00282ed480da1f",
                "isCold": false,
                "label": "Test Wallet",
                "startDate": "2022-01-01T00:00:00.000Z",
                "type": "hot",
                "balanceString": "1000000",
                "balance": 1000000,
                "confirmedBalanceString": "1000000",
                "confirmedBalance": 1000000,
                "spendableBalanceString": "1000000",
                "spendableBalance": 1000000
            }"#;

            let response: Wallet =
                serde_json::from_str(response_json).expect("Failed to deserialize response");

            // Verify basic wallet properties
            assert_eq!(response.id, "59cd72485007a239fb00282ed480da1f");
            assert_eq!(response.label, "Test Wallet");
            assert!(!response.deleted);
            assert!(!response.is_cold);
            assert_eq!(response.approvals_required, 1);
            assert_eq!(response.coin.len(), 1);
            assert_eq!(response.coin[0], "btc");

            // Verify balance fields
            assert_eq!(response.balance, Some(BaseAmount::from(1000000i128)));
            assert_eq!(
                response.confirmed_balance,
                Some(BaseAmount::from(1000000i128))
            );
            assert_eq!(
                response.confirmed_balance,
                Some(BaseAmount::from(1000000i128))
            );
            assert_eq!(
                response.spendable_balance,
                Some(BaseAmount::from(1000000i128))
            );
            assert_eq!(
                response.spendable_balance,
                Some(BaseAmount::from(1000000i128))
            );
        }

        #[test]
        fn test_minimal_response() {
            // Test minimal response with only required fields
            let response_json = r#"{
                "allowBackupKeySigning": true,
                "approvalsRequired": 2,
                "coin": ["eth"],
                "coinSpecific": {},
                "deleted": false,
                "disableTransactionNotifications": false,
                "hasLargeNumberOfAddresses": false,
                "id": "wallet456",
                "isCold": true,
                "label": "Cold Wallet",
                "startDate": "2023-06-15T10:30:00.000Z"
            }"#;

            let response: Wallet = serde_json::from_str(response_json)
                .expect("Failed to deserialize minimal response");

            assert_eq!(response.id, "wallet456");
            assert_eq!(response.label, "Cold Wallet");
            assert!(response.is_cold);
            assert!(response.allow_backup_key_signing);
            assert_eq!(response.approvals_required, 2);

            // Optional fields should be None
            assert_eq!(response.balance, None);
            assert_eq!(response.balance, None);
            assert_eq!(response.staking_balance, None);
            assert_eq!(response.reward_balance, None);
        }

        #[test]
        fn test_response_with_staking_balances() {
            // Test response with staking balance fields
            let response_json = r#"{
                "allowBackupKeySigning": false,
                "approvalsRequired": 1,
                "coin": ["eth"],
                "coinSpecific": {},
                "deleted": false,
                "disableTransactionNotifications": false,
                "hasLargeNumberOfAddresses": false,
                "id": "staking_wallet",
                "isCold": false,
                "label": "Staking Wallet",
                "startDate": "2023-01-01T00:00:00.000Z",
                "stakingBalanceString": "5000000",
                "rewardBalanceString": "50000"
            }"#;

            let response: Wallet = serde_json::from_str(response_json)
                .expect("Failed to deserialize staking response");

            assert_eq!(response.id, "staking_wallet");
            assert_eq!(response.label, "Staking Wallet");
            assert_eq!(
                response.staking_balance,
                Some(BaseAmount::from(5000000i128))
            );
            assert_eq!(response.reward_balance, Some(BaseAmount::from(50000i128)));
        }
    }

    mod path_generation {
        use super::*;

        #[test]
        fn test_btc_wallet_path() {
            let request = GetWalletByIdCoin::builder()
                .coin("btc")
                .wallet_id("59cd72485007a239fb00282ed480da1f")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/btc/wallet/59cd72485007a239fb00282ed480da1f");
        }

        #[test]
        fn test_eth_wallet_path() {
            let request = GetWalletByIdCoin::builder()
                .coin("eth")
                .wallet_id("wallet123")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/eth/wallet/wallet123");
        }

        #[test]
        fn test_special_characters_in_path() {
            let request = GetWalletByIdCoin::builder()
                .coin("tsol:usdcv2")
                .wallet_id("test-wallet_123")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/tsol:usdcv2/wallet/test-wallet_123");
        }
    }
}
