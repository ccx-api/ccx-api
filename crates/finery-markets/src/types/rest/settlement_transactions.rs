use crate::types::SettlementTransaction;

pub const API_SETTLEMENT_TRANSACTIONS: &str = "api/settlementTransactions";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementTransactionsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementTransactionsResponse {
    pub incoming_transactions: Vec<SettlementTransaction>,
    pub outgoing_transactions: Vec<SettlementTransaction>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_settlement_transactions() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<SettlementTransactionsRequest>(json);

        let json = r#"
        [
            [
                [
                    7,
                    "BTC",
                    10000000,
                    123456789,
                    "some comment",
                    1558050900000,
                    "a79f290b3a0928c",
                    1558050900000,
                    0,
                    0,
                    0,
                    100
                ]
            ],
            [
                
            ]
        ]
        "#;
        test_serde_response::<SettlementTransactionsResponse>(json);
        test_serde_response_err::<SettlementTransactionsResponse>();
    }
}
