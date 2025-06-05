use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::proto::{Request, SignedRequest};
use crate::types::derive::Request;
use crate::types::rate_limits::RateLimitType;
use crate::types::wallet::Wallet;

/// Request to get wallet information by ID
#[apply(Request)]
pub struct GetWalletById {
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

    /// Include detailed balance information (balanceString, confirmedBalanceString, spendableBalanceString)
    expand_balance: Option<bool>,
}

impl Request for GetWalletById {
    type Response = Wallet;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        let wallet_id = &self.wallet_id;

        format!("/api/v2/wallet/{wallet_id}").into()
    }
}

impl SignedRequest for GetWalletById {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    mod request_serialization {
        use super::*;

        #[test]
        fn test_basic_request_no_parameters() {
            // Test basic request with no optional parameters
            let request = GetWalletById::builder()
                .wallet_id("test_wallet_id".to_string())
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
            let request = GetWalletById::builder()
                .wallet_id("test_wallet_id".to_string())
                .all_tokens(true)
                .unspent_count(true)
                .include_rbf(false)
                .expand_advanced_whitelist(true)
                .include_staking_balances(true)
                .include_balance(false)
                .expand_balance(true)
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
                "includeBalance": false,
                "expandBalance": true
            });

            assert_eq!(parsed, expected);
        }

        #[test]
        fn test_partial_parameters() {
            // Test request with only some parameters set
            let request = GetWalletById::builder()
                .wallet_id("partial_test".to_string())
                .all_tokens(true)
                .include_staking_balances(false)
                .expand_balance(true)
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

            let expected = serde_json::json!({
                "allTokens": true,
                "includeStakingBalances": false,
                "expandBalance": true
            });

            assert_eq!(parsed, expected);
        }
    }

    mod response_deserialization {
        use super::*;
        use crate::types::BaseAmount;

        #[test]
        fn test_successful_response() {
            // Sample JSON response based on BitGo API documentation
            let response_json = r#"{
                "id": "59cd72485007a239fb00282ed480da1f",
                "coin": ["btc"],
                "label": "Test Wallet",
                "m": 2,
                "n": 3,
                "keys": [
                    "585951a5df8380e0e304a553",
                    "585951a5df8380e0e30d645c",
                    "585951a5df8380e0e30b6147"
                ],
                "users": [
                    {
                        "user": "59cd72485007a239fb00282ed480da1f",
                        "permissions": ["admin", "view", "spend"]
                    }
                ],
                "enterprise": "59cd72485007a239fb00282ed480da1f",
                "type": "hot",
                "multisigType": "onchain",
                "tags": [],
                "freeze": {},
                "deleted": false,
                "approvalsRequired": 1,
                "isCold": false,
                "recoverable": false,
                "coinSpecific": {},
                "admin": {
                    "policy": {
                        "id": "59cd72485007a239fb00282ed480da1f",
                        "version": 0,
                        "date": "2023-01-01T00:00:00.000Z",
                        "rules": []
                    }
                },
                "clientFlags": [],
                "walletFlags": [],
                "allowBackupKeySigning": false,
                "startDate": "2023-01-01T00:00:00.000Z",
                "buildDefaults": {},
                "customChangeKeySignatures": {},
                "hasLargeNumberOfAddresses": false,
                "disableTransactionNotifications": false,
                "config": {}
            }"#;

            let response: Wallet =
                serde_json::from_str(response_json).expect("Failed to deserialize response");

            assert_eq!(response.id, "59cd72485007a239fb00282ed480da1f");
            assert_eq!(response.coin, vec!["btc"]);
            assert_eq!(response.label, "Test Wallet");
            assert_eq!(response.m, Some(2));
            assert_eq!(response.n, Some(3));
        }

        #[test]
        fn test_response_with_balances() {
            // Test response that includes balance information
            let response_json = r#"{
                "id": "59cd72485007a239fb00282ed480da1f",
                "coin": ["btc"],
                "label": "Test Wallet",
                "m": 2,
                "n": 3,
                "keys": [
                    "585951a5df8380e0e304a553"
                ],
                "users": [],
                "type": "hot",
                "multisigType": "onchain",
                "tags": [],
                "freeze": {},
                "deleted": false,
                "approvalsRequired": 1,
                "isCold": false,
                "recoverable": false,
                "coinSpecific": {},
                "admin": {
                    "policy": {
                        "id": "59cd72485007a239fb00282ed480da1f",
                        "version": 0,
                        "date": "2023-01-01T00:00:00.000Z",
                        "rules": []
                    }
                },
                "clientFlags": [],
                "walletFlags": [],
                "allowBackupKeySigning": false,
                "startDate": "2023-01-01T00:00:00.000Z",
                "buildDefaults": {},
                "customChangeKeySignatures": {},
                "hasLargeNumberOfAddresses": false,
                "disableTransactionNotifications": false,
                "config": {},
                "balanceString": "1000000000000000",
                "confirmedBalanceString": "1000000000000000",
                "spendableBalanceString": "1000000000000000"
            }"#;

            let response: Wallet =
                serde_json::from_str(response_json).expect("Failed to deserialize response");

            assert_eq!(
                response.balance,
                Some(BaseAmount::from(1000000000000000i128))
            );
            assert_eq!(
                response.confirmed_balance,
                Some(BaseAmount::from(1000000000000000i128))
            );
            assert_eq!(
                response.spendable_balance,
                Some(BaseAmount::from(1000000000000000i128))
            );
        }
    }

    mod path_generation {
        use super::*;

        #[test]
        fn test_path_construction() {
            let request = GetWalletById::builder()
                .wallet_id("wallet123".to_string())
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/wallet/wallet123");
        }

        #[test]
        fn test_path_construction_with_special_chars() {
            let request = GetWalletById::builder()
                .wallet_id("wallet_123_test".to_string())
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/wallet/wallet_123_test");
        }
    }
}
