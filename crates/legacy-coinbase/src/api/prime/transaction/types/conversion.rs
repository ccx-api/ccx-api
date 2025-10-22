use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Conversion {
    /// The activity ID for the conversion.
    pub activity_id: Uuid,
    /// The currency symbol to convert from.
    pub source_symbol: Atom,
    /// The currency symbol to convert to.
    pub destination_symbol: Atom,
    /// The amount in whole units to convert.
    pub amount: Decimal,
    /// The UUID of the destination wallet.
    pub destination: Uuid,
    /// The UUID of the source wallet.
    pub source: Uuid,
    /// The UUID of the conversion transaction.
    pub transaction_id: Uuid,
}

#[cfg(test)]
mod tests {
    use ccx_api_lib::dec;
    use uuid1::uuid;

    use super::*;

    #[test]
    fn it_deserializes_doc() {
        let json = r#"{
            "activity_id": "e84255eb-2e21-439e-a1d0-f5dd1e1292b9",
            "source_symbol": "USD",
            "destination_symbol": "USDC",
            "amount": "50.50",
            "destination": "e84255eb-2e21-439e-a1d0-f5dd1e1292b9",
            "source": "e84255eb-2e21-439e-a1d0-f5dd1e1292b9",
            "transaction_id": "e84255eb-2e21-439e-a1d0-f5dd1e1292b9"
        }"#;
        let sample = Conversion {
            activity_id: uuid!("e84255eb-2e21-439e-a1d0-f5dd1e1292b9"),
            source_symbol: "USD".into(),
            destination_symbol: "USDC".into(),
            amount: dec!(50.50),
            destination: uuid!("e84255eb-2e21-439e-a1d0-f5dd1e1292b9"),
            source: uuid!("e84255eb-2e21-439e-a1d0-f5dd1e1292b9"),
            transaction_id: uuid!("e84255eb-2e21-439e-a1d0-f5dd1e1292b9"),
        };
        let deserialized: Conversion = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, sample);
    }

    // #[test]
    // fn it_deserializes_live_response_1() {
    //     let json = r#"{
    //     }"#;
    //     let sample = Conversion {
    //     };
    //     let deserialized: Conversion = serde_json::from_str(json).unwrap();
    //     assert_eq!(deserialized, sample);
    // }
}
