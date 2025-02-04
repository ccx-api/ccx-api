pub const API_ADD_OUTGOING_SETTLEMENT_TRANSACTION: &str = "api/addOutgoingSettlementTransaction";

use crate::types::ClientId;
use crate::types::DealId;
use crate::types::Size;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddOutgoingSettlementTransactionRequest {
    /// counterpartyId
    /// Efx::ClientId
    /// Counterparty Id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    /// currency
    /// string
    /// Currency name
    pub currency: String,
    /// amount
    /// Efx::Size
    /// Amount
    pub amount: Size,
    /// comment
    /// string
    /// Comment
    pub comment: String,
    /// fee
    /// Efx::Size
    /// Optional. Network fee
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<Size>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddOutgoingSettlementTransactionResponse {
    #[serde(rename = "settlementTransactionId")]
    pub settlement_transaction_id: DealId,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_add_outgoing_settlement_transaction() {
        let json = r#"
        {
            "counterpartyId": 4,
            "currency": "USD",
            "amount": 100000000000,
            "comment": "I am going to send $1000 transaction",
            "fee": 10000
        }
        "#;
        test_serde_value_type::<AddOutgoingSettlementTransactionRequest>(json);

        let json = r#"
        {
            "settlementTransactionId": 123456789
        }
        "#;
        test_serde_response::<AddOutgoingSettlementTransactionResponse>(json);
        test_serde_response_err::<AddOutgoingSettlementTransactionResponse>();
    }
}
