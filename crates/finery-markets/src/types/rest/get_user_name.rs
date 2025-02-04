use crate::types::ClientId;

#[deprecated = "API Method \"getUsername\" was replaced with \"getSubaccounts\"."]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetUserNameRequest {
    #[serde(rename = "id")]
    pub counterparty_id: ClientId,
}

#[deprecated = "API Method \"getUsername\" was replaced with \"getSubaccounts\"."]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetUserNameResponse {
    pub username: String,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_get_user_name() {
        let json = r#"
        {
            "id": 87
        }
        "#;
        test_serde_value_type::<GetUserNameRequest>(json);

        let json = r#"
        {
            "username": "username"
        }
        "#;
        test_serde_response::<GetUserNameResponse>(json);
        test_serde_response_err::<GetUserNameResponse>();
    }
}
