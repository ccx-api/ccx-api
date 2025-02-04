use super::TransactionStatus;
use super::TransactionType;
use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub portfolio_id: Uuid,
    pub r#type: TransactionType,
    pub status: TransactionStatus,
    pub symbol: Atom,
    pub created_at: DtCoinbasePrime,
    pub completed_at: Option<DtCoinbasePrime>,
    pub amount: Decimal,
    pub transfer_from: TransactionTransfer,
    pub transfer_to: TransactionTransfer,
    pub fees: Decimal,
    pub fee_symbol: Atom,
    pub blockchain_ids: Vec<String>,
    pub transaction_id: String,
    #[serde(default, with = "maybe_str")]
    pub destination_symbol: Option<Atom>,
    pub estimated_network_fees: Option<TransactionEstimatedNetworkFees>,
    #[serde(default, with = "maybe_str")]
    pub network: Option<String>,
    pub estimated_asset_changes: Vec<EstimatedAssetChange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TransactionTransfer {
    pub r#type: Atom,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TransactionEstimatedNetworkFees {
    pub lower_bound: Decimal,
    pub upper_bound: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct EstimatedAssetChange {
    pub r#type: String,
    pub symbol: Atom,
    pub amount: Decimal,
    pub collection: Collection,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Collection {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Item {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use ccx_coinbase_examples_util::d;

    use super::*;

    // #[test]
    // fn test_deserialize_transaction_doc() {
    //     let json = r#"{
    //         "id": "BTC-USD",
    //         "wallet_id": "cde8dd34-b6cf-4c2c-82bc-5f86adacc868",
    //         "portfolio_id": "0a66a8c0-24ea-4f18-b14f-8c9cf7c1ba40",
    //         "type": "string",
    //         "status": "string",
    //         "symbol": "BTC",
    //         "created_at": "2021-05-31T11:59:59Z",
    //         "completed_at": "2021-05-31T12:09:31Z",
    //         "amount": "100",
    //         "transfer_from": {
    //           "type": "string",
    //           "value": "0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27"
    //         },
    //         "transfer_to": {
    //           "type": "string",
    //           "value": "0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27"
    //         },
    //         "network_fees": "1.99",
    //         "fees": "4.53",
    //         "fee_symbol": "USD",
    //         "blockchain_ids": [
    //           "string"
    //         ],
    //         "transaction_id": "A1B2C3D4",
    //         "destination_symbol": "USD",
    //         "estimated_network_fees": {
    //           "lower_bound": "1.99",
    //           "upper_bound": "2.99"
    //         },
    //         "network": "ethereum-mainnet",
    //         "estimated_asset_changes": [
    //           {
    //             "type": "string",
    //             "symbol": "ETH",
    //             "amount": "100",
    //             "collection": {
    //               "name": "string"
    //             },
    //             "item": {
    //               "name": "string"
    //             }
    //           }
    //         ]
    //       }"#;
    //     let sample = Transaction {
    //         id: Uuid::parse_str("BTC-USD").unwrap(),
    //         wallet_id: Uuid::parse_str("cde8dd34-b6cf-4c2c-82bc-5f86adacc868").unwrap(),
    //         portfolio_id: Uuid::parse_str("0a66a8c0-24ea-4f18-b14f-8c9cf7c1ba40").unwrap(),
    //         r#type: "string".to_string(),
    //         status: "string".to_string(),
    //         symbol: Atom::from("BTC"),
    //         created_at: DtCoinbasePrime::from("2021-05-31T11:59:59Z"),
    //         completed_at: DtCoinbasePrime::from("2021-05-31T12:09:31Z"),
    //         amount: Decimal::from(100),
    //         transfer_from: TransactionTransfer {
    //             r#type: "string".to_string(),
    //             value: Uuid::parse_str("0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27").unwrap(),
    //         },
    //         transfer_to: TransactionTransfer {
    //             r#type: "string".to_string(),
    //             value: Uuid::parse_str("0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27").unwrap(),
    //         },
    //         fees: "4.53".to_string(),
    //         fee_symbol: Atom::from("USD"),
    //         blockchain_ids: vec!["string".to_string()],
    //         transaction_id: "A1B2C3D4".to_string(),
    //         destination_symbol: Atom::from("USD"),
    //         estimated_network_fees: TransactionEstimatedNetworkFees {
    //             lower_bound: Decimal::from(1.99),
    //             upper_bound: Decimal::from(2.99),
    //         },
    //         network: "ethereum-mainnet".to_string(),
    //         estimated_asset_changes: vec![EstimatedAssetChange {
    //             r#type: "string".to_string(),
    //             symbol: Atom::from("ETH"),
    //             amount: Decimal::from(100),
    //             collection: Collection {
    //                 name: "string".to_string(),
    //             },
    //             item: Item {
    //                 name: "string".to_string(),
    //             },
    //         }],
    //     };
    //     let transaction: Transaction = serde_json::from_str(json).unwrap();
    //     assert_eq!(transaction, sample);
    // }

    #[test]
    fn test_deserialize_transaction_live_1() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "wallet_id": "12345678-0000-0000-0000-000000000000",
            "portfolio_id": "12345678-0000-0000-0000-000000000000",
            "type": "WITHDRAWAL",
            "status": "TRANSACTION_DONE",
            "symbol": "USDT",
            "created_at": "2024-01-23T12:34:56.123Z",
            "completed_at": "2024-01-23T12:37:12Z",
            "amount": "-10",
            "transfer_from": {
                "type": "WALLET",
                "value": "12345678-0000-0000-0000-000000000000"
            },
            "transfer_to": {
                "type": "ADDRESS",
                "value": "0x1234567812345678123456781234567812345678"
            },
            "network_fees": "0",
            "fees": "0",
            "fee_symbol": "ETH",
            "blockchain_ids": [
                "0x1234567812345678123456781234567812345678123456781234567812345678"
            ],
            "transaction_id": "A1B2C3D4",
            "destination_symbol": "",
            "estimated_network_fees": null,
            "network": "",
            "estimated_asset_changes": [],
            "metadata": null
        }"#;
        let sample = Transaction {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            wallet_id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            portfolio_id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            r#type: TransactionType::Withdrawal,
            status: TransactionStatus::Done,
            symbol: Atom::from("USDT"),
            created_at: DtCoinbasePrime::parse_from_str("2024-01-23T12:34:56.123Z").unwrap(),
            completed_at: Some(DtCoinbasePrime::parse_from_str("2024-01-23T12:37:12Z").unwrap()),
            amount: d("-10"),
            transfer_from: TransactionTransfer {
                r#type: "WALLET".into(),
                value: "12345678-0000-0000-0000-000000000000".to_string(),
            },
            transfer_to: TransactionTransfer {
                r#type: "ADDRESS".into(),
                value: "0x1234567812345678123456781234567812345678".to_string(),
            },
            fees: d("0"),
            fee_symbol: Atom::from("ETH"),
            blockchain_ids: vec![
                "0x1234567812345678123456781234567812345678123456781234567812345678".to_string(),
            ],
            transaction_id: "A1B2C3D4".to_string(),
            destination_symbol: None,
            estimated_network_fees: None,
            network: None,
            estimated_asset_changes: vec![],
        };
        let transaction: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(transaction, sample);
    }

    #[test]
    fn test_deserialize_transaction_live_2() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "wallet_id": "12345678-0000-0000-0000-000000000000",
            "portfolio_id": "12345678-0000-0000-0000-000000000000",
            "type": "WITHDRAWAL",
            "status": "TRANSACTION_CREATED",
            "symbol": "USDT",
            "created_at": "2024-01-23T12:34:56.123Z",
            "completed_at": null,
            "amount": "-10",
            "transfer_from": {
                "type": "WALLET",
                "value": "12345678-0000-0000-0000-000000000000"
            },
            "transfer_to": {
                "type": "ADDRESS",
                "value": "0x1234567812345678123456781234567812345678"
            },
            "network_fees": "0",
            "fees": "0",
            "fee_symbol": "ETH",
            "blockchain_ids": [],
            "transaction_id": "A1B2C3D4",
            "destination_symbol": "",
            "estimated_network_fees": null,
            "network": "",
            "estimated_asset_changes": [],
            "metadata": null
        }"#;
        let sample = Transaction {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            wallet_id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            portfolio_id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            r#type: TransactionType::Withdrawal,
            status: TransactionStatus::Created,
            symbol: Atom::from("USDT"),
            created_at: DtCoinbasePrime::parse_from_str("2024-01-23T12:34:56.123Z").unwrap(),
            completed_at: None,
            amount: d("-10"),
            transfer_from: TransactionTransfer {
                r#type: "WALLET".into(),
                value: "12345678-0000-0000-0000-000000000000".to_string(),
            },
            transfer_to: TransactionTransfer {
                r#type: "ADDRESS".into(),
                value: "0x1234567812345678123456781234567812345678".to_string(),
            },
            fees: d("0"),
            fee_symbol: Atom::from("ETH"),
            blockchain_ids: vec![],
            transaction_id: "A1B2C3D4".to_string(),
            destination_symbol: None,
            estimated_network_fees: None,
            network: None,
            estimated_asset_changes: vec![],
        };
        let transaction: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(transaction, sample);
    }
}
