use crate::api::exchange::prelude::*;

/// Represents a requested withdraw to address object from Coinbase Exchange/Pro API.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RequestedWithdrawal {
    /// The unique identifier of the requested withdraw to address.
    pub id: Uuid,
    /// The amount of the requested withdraw to address, as a decimal.
    pub amount: Decimal,
    /// The currency of the requested withdraw to address.
    pub currency: Atom,
    /// The time at which the withdraw to address is scheduled to be paid out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_at: Option<DtCoinbaseEx>,
    /// The fee associated with the requested withdraw to address.
    pub fee: Option<Decimal>,
    /// The subtotal amount of the requested withdraw to address.
    pub subtotal: Option<Decimal>,
    /// The network of the requested withdraw to address (undocumented).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

#[cfg(test)]
mod tests {
    use ccx_api_lib::dec;

    use super::*;

    #[test]
    fn test_deserialize_live() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "amount": "3.00000000",
            "currency": "USDT",
            "fee": 2.278178,
            "subtotal": 0.721822,
            "network": "ethereum"
        }"#;
        let sample = RequestedWithdrawal {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            amount: dec!(3.00000000),
            currency: Atom::from("USDT"),
            payout_at: None,
            fee: Some(dec!(2.278178)),
            subtotal: Some(dec!(0.721822)),
            network: Some("ethereum".to_string()),
        };
        let withdrawal: RequestedWithdrawal = serde_json::from_str(json).unwrap();
        assert_eq!(withdrawal, sample);
    }

    #[test]
    fn test_deserialize_live_2() {
        let json = r#"{
            "id": "12345678-0000-0000-0000-000000000000",
            "amount": "10.00000000",
            "currency": "USDT"
        }"#;
        let sample = RequestedWithdrawal {
            id: Uuid::parse_str("12345678-0000-0000-0000-000000000000").unwrap(),
            amount: dec!(10.00000000),
            currency: Atom::from("USDT"),
            payout_at: None,
            fee: None,
            subtotal: None,
            network: None,
        };
        let withdrawal: RequestedWithdrawal = serde_json::from_str(json).unwrap();
        assert_eq!(withdrawal, sample);
    }
}
