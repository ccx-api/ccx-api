pub const API_DEL_OUTGOING_SETTLEMENT_TRANSACTION: &str = "api/delOutgoingSettlementTransaction";

use crate::types::OrderId;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelOutgoingSettlementTransactionRequest {
    /// transactionId
    /// Efx::OrderId
    /// Transaction's order id
    #[serde(rename = "transactionId")]
    pub transaction_id: OrderId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelOutgoingSettlementTransactionResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del_outgoing_settlement_transaction() {
        let json = r#"
        {
            "transactionId": 123456789
        }
        "#;
        test_serde_value_type::<DelOutgoingSettlementTransactionRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<DelOutgoingSettlementTransactionResponse>(json);
        test_serde_response_err::<DelOutgoingSettlementTransactionResponse>();
    }
}
