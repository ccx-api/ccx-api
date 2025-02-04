pub const API_DEL_LIMIT: &str = "api/delLimit";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelLimitRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DelLimitResponse {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_del_limit() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<DelLimitRequest>(json);

        let json = r#"
        {
            "error": 0
        }
        "#;
        test_serde_response::<DelLimitResponse>(json);
        test_serde_response_err::<DelLimitResponse>();
    }
}
