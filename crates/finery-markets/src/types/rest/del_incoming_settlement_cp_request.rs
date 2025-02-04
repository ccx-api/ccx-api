pub const API_DEL_INCOMING_SETTLEMENT_CP_REQUEST: &str = "api/delIncomingSettlementCPRequest";

use crate::types::ClientId;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelIncomingSettlementCPRequest {
    // counterpartyId
    // Efx::ClientId
    // Request sender id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    // currency
    // string
    // Currency name
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelIncomingSettlementCPResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del_incoming_settlement_cp_request() {
        let json = r#"
        {
            "counterpartyId": 4,
            "currency": "USD"
        }
        "#;
        test_serde_value_type::<DelIncomingSettlementCPRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<DelIncomingSettlementCPResponse>(json);
        test_serde_response_err::<DelIncomingSettlementCPResponse>();
    }
}
