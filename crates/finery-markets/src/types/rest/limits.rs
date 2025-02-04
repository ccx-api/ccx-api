use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::Size;

pub const API_LIMITS: &str = "api/limits";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LimitsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LimitsResponse(pub Vec<Limit>);

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct Limit {
    /// 0
    /// string
    /// Currency name
    pub currency_name: String,
    /// 1
    /// Efx::Size
    /// Net limit
    pub net_limit: Size,
    /// 2
    /// Efx::Size
    /// Gross limit
    pub gross_limit: Size,
    /// 3
    /// Efx::Size
    /// Net limit utilisation
    pub net_limit_utilisation: Size,
    /// 4
    /// Efx::Size
    /// Gross limit utilisation
    pub gross_limit_utilisation: Size,
    /// 5
    /// unsigned int16
    /// Reserved
    pub reserved: u16,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_limits() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<LimitsRequest>(json);

        let json = r#"
        [
            [
                "BTC",
                10000000,
                100000000,
                0,
                0,
                0
            ]
        ]
        "#;
        test_serde_response::<LimitsResponse>(json);
        test_serde_response_err::<LimitsResponse>();
    }
}
