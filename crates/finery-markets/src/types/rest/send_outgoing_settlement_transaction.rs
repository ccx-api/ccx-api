pub const API_SEND_OUTGOING_SETTLEMENT_TRANSACTION: &str = "api/sendOutgoingSettlementTransaction";

use crate::types::OrderId;
use crate::types::Size;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SendOutgoingSettlementTransactionRequest {
    /// transactionId
    ///
    /// Efx::OrderId
    ///
    /// Transaction's order id
    #[serde(rename = "transactionId")]
    pub transaction_id: OrderId,

    /// txId
    ///
    /// string
    ///
    /// Transaction's external id (for example blockchain txId)
    #[serde(rename = "txId")]
    pub tx_id: String,

    /// fee
    ///
    /// Efx::Size
    ///
    /// Optional. Network fee.
    ///
    /// The amount that the recipient will receive to their wallet or
    /// custodian account will be smaller than the one you have specified
    /// as transaction amount by network fee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<Size>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SendOutgoingSettlementTransactionResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_send_outgoing_settlement_transaction() {
        let json = r#"
        {
            "transactionId": 123456789,
            "txId": "1A2B3C1A2B3C1A2B3C1A2B3C1A2B3C1A2B3C1A2B3C"
        }
        "#;
        test_serde_value_type::<SendOutgoingSettlementTransactionRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<SendOutgoingSettlementTransactionResponse>(json);
        test_serde_response_err::<SendOutgoingSettlementTransactionResponse>();
    }
}
