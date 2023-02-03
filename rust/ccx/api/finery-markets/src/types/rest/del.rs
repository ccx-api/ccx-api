pub const API_DEL: &str = "api/del";

use crate::types::ClientOrderId;
use crate::types::OrderId;
use crate::types::Size;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelRequest {
    /// orderId
    /// Efx::OrderId
    /// Id of order to delete (either this or clientOrderId should be specified)
    #[serde(rename = "orderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// clientOrderId
    /// Efx::ClientOrderId
    /// Client Id of order to delete (either this or orderId should be specified)
    #[serde(rename = "clientOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelResponse {
    /// id
    /// Efx::OrderId
    /// Order Id
    pub id: OrderId,
    /// remainingSize
    /// Efx::Size
    /// Remaining Order size on removal
    #[serde(rename = "remainingSize")]
    pub remaining_size: Size,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del() {
        let json = r#"
        {
            "orderId": 1235
        }
        "#;
        test_serde_value_type::<DelRequest>(json);

        let json = r#"
        {
            "id": 1235,
            "remainingSize": 8000000
        }
        "#;
        test_serde_response::<DelResponse>(json);
        test_serde_response_err::<DelResponse>();
    }
}
