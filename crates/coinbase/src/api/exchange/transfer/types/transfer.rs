use crate::api::exchange::prelude::*;
use crate::api::exchange::TransferDetails;
use crate::api::exchange::TransferType;

/// Represents a transfer object from Coinbase Exchange/Pro API.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transfer {
    /// The unique identifier of the transfer.
    pub id: Uuid,
    /// The type of the transfer (deposit, withdraw, internal_deposit, or internal_withdraw).
    pub r#type: TransferType,
    /// The time at which the transfer was created.
    pub created_at: DtCoinbaseEx,
    /// The time at which the transfer was completed (if applicable).
    pub completed_at: Option<DtCoinbaseEx>,
    /// The time at which the transfer was canceled (if applicable).
    pub canceled_at: Option<DtCoinbaseEx>,
    /// The time at which the transfer was processed (if applicable).
    pub processed_at: Option<DtCoinbaseEx>,
    /// The amount of the transfer, as a decimal.
    pub amount: Decimal,
    /// Additional details about the transfer.
    pub details: TransferDetails,
    /// A nonce assigned by the user for their own reference.
    pub user_nonce: Option<String>,
    /// The currency of the transfer, e.g. "BTC" or "USD".
    pub currency: Atom,
}

#[cfg(test)]
mod tests {
    use ccx_api_lib::dec;

    use super::*;

    #[test]
    fn test_deserialize_live() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "type": "withdraw",
            "created_at": "2024-04-26 01:23:45.819928+00",
            "completed_at": "2024-04-26 01:23:45.50722+00",
            "account_id": "12345678-0000-0000-0000-000000000000",
            "user_id": "636263626362636263626362",
            "amount": "123456.15000000",
            "details": {
                "fee": "1.234567",
                "network": "ethereum",
                "subtotal": "123456.123456",
                "sent_to_address": "0x1324576813245768132457681324576813245768",
                "coinbase_account_id": "12345678-0000-0000-0000-000000000000",
                "crypto_transaction_hash": "08cd6790c2cac45494e58dbf23af094f89d8f9ab2a34d4b012ef2acb18285c99",
                "tx_service_transaction_id": "1234567890",
                "coinbase_payment_method_id": ""
            },
            "canceled_at": null,
            "processed_at": "2024-04-26 01:23:45.123456+00",
            "user_nonce": "1234567801230",
            "idem": "12345678-0000-0000-0000-000000000000",
            "profile_id": "12345678-0000-0000-0000-000000000000",
            "currency": "USDT"
        }"#;
        let sample = Transfer {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            r#type: TransferType::Withdraw,
            created_at: DtCoinbaseEx::parse_from_str("2024-04-26 01:23:45.819928+00").unwrap(),
            completed_at: Some(
                DtCoinbaseEx::parse_from_str("2024-04-26 01:23:45.50722+00").unwrap(),
            ),
            canceled_at: None,
            processed_at: Some(
                DtCoinbaseEx::parse_from_str("2024-04-26 01:23:45.123456+00").unwrap(),
            ),
            amount: dec!(123456.15000000),
            details: TransferDetails {
                fee: Some(dec!(1.234567)),
                network: Some("ethereum".to_string()),
                subtotal: Some(dec!(123456.123456)),
                sent_to_address: Some("0x1324576813245768132457681324576813245768".to_string()),
                coinbase_account_id: Some(
                    Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
                ),
                crypto_transaction_hash: Some(
                    "08cd6790c2cac45494e58dbf23af094f89d8f9ab2a34d4b012ef2acb18285c99".to_string(),
                ),
                tx_service_transaction_id: Some("1234567890".to_string()),
                coinbase_transaction_id: None,
                coinbase_payment_method_id: None,
            },
            user_nonce: Some("1234567801230".to_string()),
            currency: "USDT".into(),
        };
        let decoded = serde_json::from_str::<Transfer>(json).unwrap();
        assert_eq!(decoded, sample);
    }

    #[test]
    fn test_deserialize_live_2() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "type": "withdraw",
            "created_at": "2024-02-20 11:07:29.245081+00",
            "completed_at": "2024-02-20 11:07:30.651189+00",
            "account_id": "12345678-0000-0000-0000-000000000000",
            "user_id": "636263626362636263626362",
            "amount": "10.00000000",
            "details": {
                "coinbase_account_id": "12345678-0000-0000-0000-000000000000",
                "coinbase_transaction_id": "123456789012345678901234",
                "coinbase_payment_method_id": ""
            },
            "canceled_at": null,
            "processed_at": null,
            "user_nonce": null,
            "idem": null,
            "profile_id": "12345678-0000-0000-0000-000000000000",
            "currency": "USDT"
        }"#;
        let sample = Transfer {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            r#type: TransferType::Withdraw,
            created_at: DtCoinbaseEx::parse_from_str("2024-02-20 11:07:29.245081+00").unwrap(),
            completed_at: Some(
                DtCoinbaseEx::parse_from_str("2024-02-20 11:07:30.651189+00").unwrap(),
            ),
            canceled_at: None,
            processed_at: None,
            amount: dec!(10.00000000),
            details: TransferDetails {
                fee: None,
                network: None,
                subtotal: None,
                sent_to_address: None,
                coinbase_account_id: Some(
                    Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
                ),
                crypto_transaction_hash: None,
                tx_service_transaction_id: None,
                coinbase_transaction_id: Some("123456789012345678901234".to_string()),
                coinbase_payment_method_id: None,
            },
            user_nonce: None,
            currency: "USDT".into(),
        };
        let decoded = serde_json::from_str::<Transfer>(json).unwrap();
        assert_eq!(decoded, sample);
    }

    #[test]
    fn test_deserialize_live_3() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "type": "withdraw",
            "created_at": "2024-04-08 12:34:56.123456+00",
            "completed_at": null,
            "account_id": "12345678-0000-0000-0000-000000000000",
            "user_id": "636263626362636263626362",
            "amount": "10.00000000",
            "details": {
                "fee": "9.268372",
                "network": "ethereum",
                "subtotal": "0.731628",
                "sent_to_address": "0x1324576813245768132457681324576813245768",
                "coinbase_account_id": "12345678-0000-0000-0000-000000000000",
                "coinbase_payment_method_id": ""
            },
            "canceled_at": null,
            "processed_at": null,
            "user_nonce": "10",
            "idem": "12345678-0000-0000-0000-000000000000",
            "profile_id": "12345678-0000-0000-0000-000000000000",
            "currency": "USDT"
        }"#;
        let sample = Transfer {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            r#type: TransferType::Withdraw,
            created_at: DtCoinbaseEx::parse_from_str("2024-04-08 12:34:56.123456+00").unwrap(),
            completed_at: None,
            canceled_at: None,
            processed_at: None,
            amount: dec!(10.00000000),
            details: TransferDetails {
                fee: Some(dec!(9.268372)),
                network: Some("ethereum".to_string()),
                subtotal: Some(dec!(0.731628)),
                sent_to_address: Some("0x1324576813245768132457681324576813245768".to_string()),
                coinbase_account_id: Some(
                    Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
                ),
                crypto_transaction_hash: None,
                tx_service_transaction_id: None,
                coinbase_transaction_id: None,
                coinbase_payment_method_id: None,
            },
            user_nonce: Some("10".to_string()),
            currency: "USDT".into(),
        };
        let decoded = serde_json::from_str::<Transfer>(json).unwrap();
        assert_eq!(decoded, sample);
    }
}
