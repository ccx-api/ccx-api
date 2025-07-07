use crate::types::ClientId;
use crate::types::ClientType;
use crate::types::ClientStatus;

pub const API_GET_COUNTERPARTY_INFO: &str = "api/getCounterpartyInfo";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCounterpartyInfoRequest {
    pub counterparty_id: ClientId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCounterpartyInfoResponse {
    /// Id of the connected client.
    pub client_id: ClientId,
    /// Type of the connected client.
    pub client_type: ClientType,
    /// Optional.
    /// Username of the counterparty.
    /// Visible only if the counterparty sent or accepted a connection request.
    #[serde(default)]
    pub username: Option<String>,
    /// Optional.
    /// True and present only if the counterparty is a subaccount.
    #[serde(default)]
    pub subaccount: Option<bool>,
    /// Connection status of the client.
    pub status: ClientStatus,
    /// True if the client is disabled.
    /// Trading is not allowed with disabled clients.
    pub disabled: bool,
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
            "counterpartyId": 87
        }
        "#;
        test_serde_value_type::<GetCounterpartyInfoRequest>(json);

        let json = r#"
        {
            "clientId": 87,
            "clientType": "maker",
            "username": "username",
            "subaccount": true,
            "status": "connected",
            "disabled": false
        }
        "#;
        test_serde_response::<GetCounterpartyInfoResponse>(json);
        test_serde_response_err::<GetCounterpartyInfoResponse>();
    }
}
