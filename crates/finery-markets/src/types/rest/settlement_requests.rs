use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::ClientId;
use crate::types::SettlementFlags;
use crate::types::Size;
use crate::types::Timestamp;

pub const API_SETTLEMENT_REQUESTS: &str = "api/settlementRequests";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementResponse {
    pub incoming_requests: Vec<SettlementRequestItem>,
    pub outgoing_requests: Vec<SettlementRequestItem>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct SettlementRequestItem {
    /// 0
    /// Efx::ClientId
    /// Counterparty id
    pub counterparty_id: ClientId,
    /// 1
    /// string
    /// Currency name
    pub currency_name: String,
    /// 2
    /// Efx::Flags
    /// 0 - No flags
    /// 1 - Fee paid by recipient
    pub flags: SettlementFlags,
    /// 3
    /// Efx::Size
    /// Requested amount. If zero, the full outstanding position is supposed to be settled
    pub requested_amount: Size,
    /// 4
    /// string
    /// Comment
    pub comment: String,
    /// 5
    /// Efx::Timestamp
    /// Request expiration time in milliseconds. If 0, request won't be expired.
    pub expiration_time: Timestamp,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_settlement_requests() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<SettlementRequest>(json);

        let json = r#"
        [
            [
                [
                    4,
                    "BTC",
                    0,
                    6700000000,
                    "your comment",
                    1634406457410
                ]
            ],
            [
                [
                    4,
                    "USD",
                    0,
                    0,
                    "",
                    0
                ]
            ]
        ]
        "#;
        test_serde_response::<SettlementResponse>(json);
        test_serde_response_err::<SettlementResponse>();
    }
}
