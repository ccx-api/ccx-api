use crate::types::DealId;
use crate::types::SettlementTransaction;
use crate::types::Timestamp;

pub const API_SETTLEMENT_TRANSACTION_HISTORY: &str = "api/settlementTransactionHistory";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementTransactionHistoryRequest {
    /// till
    /// Efx::DealId
    /// If specified only return settlements with lesser ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub till: Option<DealId>,
    /// from
    /// Efx::Timestamp
    /// If specified only return settlements with equal or greater timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Timestamp>,
    /// to
    /// Efx::Timestamp
    /// If specified only return settlements with lesser timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Timestamp>,
    /// limit
    /// unsigned int16
    /// Default: 250
    /// Maximum number of settlements to return (capped at 250)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementTransactionHistoryResponse(pub Vec<SettlementTransaction>);

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_settlement_transaction_history() {
        let json = r#"
        {
            "till": 12345,
            "limit": 10
        }
        "#;
        test_serde_value_type::<SettlementTransactionHistoryRequest>(json);

        let json = r#"
        [
            [
                3,
                "USD",
                1000000000,
                1234,
                "Something",
                1558050900000,
                "12ehvb324gg",
                1558067800000,
                0,
                1558091300000,
                567,
                1000
            ]
        ]
        "#;
        test_serde_response::<SettlementTransactionHistoryResponse>(json);
        test_serde_response_err::<SettlementTransactionHistoryResponse>();
    }
}
