pub const API_DEL_ALL: &str = "api/delAll";

use crate::types::Pair;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelAllRequest {
    /// instrument
    /// string
    /// Optional. Instrument filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Pair>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelAllResponse {
    /// removed
    /// unsigned int32
    /// Number of cancelled orders
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<u32>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del_all() {
        let json = r#"
        {
            "instrument": "BTC-USD"
        }
        "#;
        test_serde_value_type::<DelAllRequest>(json);

        let json = r#"
        {
            "removed": 15
        }
        "#;
        test_serde_response::<DelAllResponse>(json);
        test_serde_response_err::<DelAllResponse>();
    }
}
