pub const API_ADD_INCOMING_SETTLEMENT_REQUEST: &str = "api/addIncomingSettlementRequest";

use crate::types::ClientId;
use crate::types::SettlementFlags;
use crate::types::Size;
use crate::types::Timestamp;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddIncomingSettlementRequest {
    /// counterpartyId
    /// Efx::ClientId
    /// Counterparty id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
    /// currency
    /// string
    /// Currency name
    pub currency: String,
    /// amount
    /// Efx::Size
    /// Optional. The amount of funds that the specified counterparty is required to send. If zero or not specified, the full outstanding position is supposed to be settled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Size>,
    /// comment
    /// string
    /// Optional. Any comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// flags
    /// unsigned int16
    /// Optional
    /// 0 - no flags
    /// 1 - fee paid by recipient
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<SettlementFlags>,
    /// cancelTimestamp
    /// Efx::Timestamp
    /// Optional. Timestamp in milliseconds when the request will be automatically deleted. Could not be more than 30 days from the current date.
    /// Default value is 24 hours ahead.
    /// If set to 0 the request doesn't have expiration time.
    #[serde(rename = "cancelTimestamp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_timestamp: Option<Timestamp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddIncomingSettlementResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_add_incoming_settlement_request() {
        let json = r#"
        {
            "counterpartyId": 4,
            "currency": "USD",
            "amount": 10000000000,
            "comment": "Some text",
            "flags": 1,
            "cancelTimestamp": 1634406457410
        }
        "#;
        test_serde_value_type::<AddIncomingSettlementRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<AddIncomingSettlementResponse>(json);
        test_serde_response_err::<AddIncomingSettlementResponse>();
    }
}
