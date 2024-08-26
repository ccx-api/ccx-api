use crate::types::ClientId;
use crate::types::Timestamp;

pub const API_GET_SUBACCOUNTS: &str = "api/getSubaccounts";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetSubaccountsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetSubaccountsResponse(pub Vec<Subaccount>);

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Subaccount {
    pub info: SubaccountInfo,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: Timestamp,
    #[serde(rename = "type")]
    pub subacc_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SubaccountInfo {
    #[serde(rename = "clientId")]
    pub clinet_id: ClientId,
    pub username: String,
    pub email: String,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_get_user_name() {
        let json = r#"{}"#;
        test_serde_value_type::<GetSubaccountsRequest>(json);

        let json = r#"
        [{
            "info": {
                "clientId": 0,
                "username": "string",
                "email": "string"
            },
            "status": "active",
            "createdAt": 0,
            "type": "maker"
        }]
        "#;
        test_serde_response::<GetSubaccountsResponse>(json);
        test_serde_response_err::<GetSubaccountsResponse>();
    }
}
