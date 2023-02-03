pub const API_COMMIT_INCOMING_SETTLEMENT_TRANSACTION: &str =
    "api/commitIncomingSettlementTransaction";

use crate::types::OrderId;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CommitIncomingSettlementTransactionRequest {
    /// transactionId
    /// Efx::OrderId
    /// Transaction's order id
    #[serde(rename = "transactionId")]
    pub transaction_id: OrderId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CommitIncomingSettlementTransactionResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_commit_incoming_settlement_transaction() {
        let json = r#"
        {
            "transactionId": 123456789
        }
        "#;
        test_serde_value_type::<CommitIncomingSettlementTransactionRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<CommitIncomingSettlementTransactionResponse>(json);
        test_serde_response_err::<CommitIncomingSettlementTransactionResponse>();
    }
}
