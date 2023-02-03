use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::ClientId;
use crate::types::DealId;
use crate::types::OrderId;
use crate::types::Size;
use crate::types::Timestamp;

pub const API_SETTLEMENT_HISTORY: &str = "api/settlementHistory";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SettlementHistoryRequest {
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
pub struct SettlementHistoryResponse(pub Vec<Settlement>);

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct Settlement {
    /// 0
    /// Efx::OrderId
    /// Settlement Order Id
    pub order_id: OrderId,
    /// 1
    /// string
    /// Currency 1
    pub currency_1: String,
    /// 2
    /// string
    /// Currency 2
    pub currency_2: String,
    /// 3
    /// Efx::Size
    /// Size 1
    pub size_1: Size,
    /// 4
    /// Efx::Size
    /// Size 2
    pub size_2: Size,
    /// 5
    /// Efx::Timestamp
    /// Created At
    pub created_at: Timestamp,
    /// 6
    /// Efx::ClientId
    /// Counterparty Id
    pub counterparty_id: ClientId,
    /// 7
    /// Efx::Timestamp
    /// Settlement Moment
    pub moment: Timestamp,
    /// 8
    /// Efx::DealId
    /// Settlement Id
    pub id: DealId,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_settlement_history() {
        let json = r#"
        {
            "till": 12345,
            "limit": 10
        }
        "#;
        test_serde_value_type::<SettlementHistoryRequest>(json);

        let json = r#"
        [
            [
                1229,
                "BTC",
                "USD",
                10000000,
                100000000000,
                1558050900000,
                2,
                1558051900000,
                245
            ]
        ]
        "#;
        test_serde_response::<SettlementHistoryResponse>(json);
        test_serde_response_err::<SettlementHistoryResponse>();
    }
}
