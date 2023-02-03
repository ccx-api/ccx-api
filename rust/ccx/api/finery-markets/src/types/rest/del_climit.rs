pub const API_DEL_CLIMIT: &str = "api/delCLimit";

use crate::types::ClientId;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelCLimitRequest {
    /// counterpartyId
    /// Efx::ClientId
    /// Counterparty Id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelCLimitResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del_climit() {
        let json = r#"
        {
            "counterpartyId": 4
        }
        "#;
        test_serde_value_type::<DelCLimitRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<DelCLimitResponse>(json);
        test_serde_response_err::<DelCLimitResponse>();
    }
}
