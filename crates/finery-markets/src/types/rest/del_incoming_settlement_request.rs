pub const API_DEL_INCOMING_SETTLEMENT_REQUEST: &str = "api/delIncomingSettlementRequest";

use crate::types::ClientId;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelIncomingSettlementRequest {
    // counterpartyId
    // Efx::ClientId
    // Counterparty Id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    // currency
    // string
    // Currency name
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelIncomingSettlementResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del_incoming_settlement_request() {
        let json = r#"
        {
            "counterpartyId": 4,
            "currency": "USD"
        }
        "#;
        test_serde_value_type::<DelIncomingSettlementRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<DelIncomingSettlementResponse>(json);
        test_serde_response_err::<DelIncomingSettlementResponse>();
    }
}
