use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::ClientId;
use crate::types::Size;

pub const API_CLIMITS: &str = "api/climits";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CLimitsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CLimitsResponse(pub Vec<CLimit>);

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct CLimit {
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
    /// 6
    /// Efx::ClientId
    /// Counterparty id
    pub counterparty_id: ClientId,
    /// 7
    /// int32
    /// Taker markup
    pub taker_markup: i32,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_climits() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<CLimitsRequest>(json);

        let json = r#"
        [
            [
                "BTC",
                10000000,
                100000000,
                0,
                0,
                0,
                1,
                100
            ],
            [
                "USD",
                100000000000,
                1000000000000,
                0,
                0,
                0,
                2,
                0
            ]
        ]
        "#;
        test_serde_response::<CLimitsResponse>(json);
        test_serde_response_err::<CLimitsResponse>();
    }
}
