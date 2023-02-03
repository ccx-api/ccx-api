pub const API_SET_LIMIT: &str = "api/setLimit";

use crate::types::Size;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SetLimitRequest {
    /// currency
    /// string
    /// Currency name
    pub currency: String,
    /// netLimit
    /// Efx::Size
    /// Net limit size
    #[serde(rename = "netLimit")]
    pub net_limit: Size,
    /// grossLimit
    /// Efx::Size
    /// Gross limit size
    #[serde(rename = "grossLimit")]
    pub gross_limit: Size,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SetLimitResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_set_limit() {
        let json = r#"
        {
            "currency": "EUR",
            "netLimit": 10000000000000,
            "grossLimit": 15000000000000
        }
        "#;
        test_serde_value_type::<SetLimitRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<SetLimitResponse>(json);
        test_serde_response_err::<SetLimitResponse>();
    }
}
