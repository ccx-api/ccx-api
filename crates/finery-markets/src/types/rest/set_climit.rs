pub const API_SET_CLIMIT: &str = "api/setCLimit";

use crate::types::ClientId;
use crate::types::Size;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SetCLimitRequest {
    /// counterpartyId
    /// Efx::ClientId
    /// Counterparty Id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
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
    /// takerMarkup
    /// int32
    /// Optional. For makers only!
    /// Set markup that is apply to your raw prices and, eventually, widens spread for the taker. In % multiplied by 1e4.
    /// From -100000 to 100000, that is from 10% to -10%
    #[serde(rename = "takerMarkup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taker_markup: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SetCLimitResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_set_climit() {
        let json = r#"
        {
            "counterpartyId": 4,
            "currency": "EUR",
            "netLimit": 10000000000000,
            "grossLimit": 15000000000000,
            "takerMarkup": 100
        }
        "#;
        test_serde_value_type::<SetCLimitRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<SetCLimitResponse>(json);
        test_serde_response_err::<SetCLimitResponse>();
    }
}
