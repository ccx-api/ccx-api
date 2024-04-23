use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct RequestedWithdrawal {
    pub activity_id: Uuid,
    pub approval_url: String,
    pub symbol: Atom,
    pub amount: Decimal,
    #[serde(default, with = "maybe_str")]
    pub fee: Option<Decimal>,
    pub destination_type: String,
    pub source_type: String,
    pub blockchain_destination: BlockchainAddress,
    pub blockchain_source: BlockchainAddress,
    pub transaction_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BlockchainAddress {
    pub address: String,
    #[serde(default, with = "maybe_str")]
    pub account_identifier: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use ccx_coinbase_examples_util::d;

    use super::*;

    // #[test]
    // fn test_deserialize_withdrawal_doc() {
    //     let withdrawal = r#"{
    //         "activity_id": "string",
    //         "approval_url": "string",
    //         "symbol": "string",
    //         "amount": "string",
    //         "fee": "string",
    //         "destination_type": "string",
    //         "source_type": "string",
    //         "blockchain_destination": {
    //         "address": "string",
    //         "account_identifier": "string"
    //         },
    //         "blockchain_source": {
    //         "address": "string",
    //         "account_identifier": "string"
    //         },
    //         "transaction_id": "string"
    //     }"#;
    //     let sample = Withdrawal {
    //         activity_id: "string".into(),
    //         approval_url: "string".into(),
    //         symbol: "string".into(),
    //         amount: "string".into(),
    //         fee: "string".into(),
    //         destination_type: "string".into(),
    //         source_type: "string".into(),
    //         blockchain_destination: BlockchainAddress {
    //             address: "string".into(),
    //             account_identifier: "string".into(),
    //         },
    //         blockchain_source: BlockchainAddress {
    //             address: "string".into(),
    //             account_identifier: "string".into(),
    //         },
    //         transaction_id: "string".into(),
    //     };
    //     let withdrawal: Withdrawal = serde_json::from_str(withdrawal).unwrap();
    // }

    #[test]
    fn test_deserialize_withdrawal_live() {
        let withdrawal = r#"{
            "activity_id": "12345678-0000-0000-0000-000000000000",
            "approval_url": "https://prime.coinbase.com/portfolio/12345678-0000-0000-0000-000000000000/activity/12345678-0000-0000-0000-000000000000",
            "symbol": "USDT",
            "amount": "10",
            "fee": "",
            "destination_type": "External Address",
            "source_type": "USDT Trading Balance",
            "blockchain_destination": {
                "address": "0x1234567812345678123456781234567812345678",
                "account_identifier": ""
            },
            "blockchain_source": {
                "address": "Trading Wallet Address",
                "account_identifier": ""
            },
            "transaction_id": "12345678-0000-0000-0000-000000000000"
        }"#;
        let sample = RequestedWithdrawal {
            activity_id: "12345678-0000-0000-0000-000000000000".parse().unwrap(),
            approval_url: "https://prime.coinbase.com/portfolio/12345678-0000-0000-0000-000000000000/activity/12345678-0000-0000-0000-000000000000".into(),
            symbol: "USDT".into(),
            amount: d("10"),
            fee: None,
            destination_type: "External Address".into(),
            source_type: "USDT Trading Balance".into(),
            blockchain_destination: BlockchainAddress {
                address: "0x1234567812345678123456781234567812345678".into(),
                account_identifier: None,
            },
            blockchain_source: BlockchainAddress {
                address: "Trading Wallet Address".into(),
                account_identifier: None,
            },
            transaction_id: "12345678-0000-0000-0000-000000000000".parse().unwrap(),
        };
        let withdrawal: RequestedWithdrawal = serde_json::from_str(withdrawal).unwrap();
        assert_eq!(withdrawal, sample);
    }
}
