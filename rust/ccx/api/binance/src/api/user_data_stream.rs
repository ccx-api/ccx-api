use super::prelude::*;

pub const V1_USER_DATA_STREAM: &str = "/api/v1/userDataStream";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ListenKey {
    pub listen_key: String,
}

impl Api {
    /// Create a listenKey.
    ///
    /// Start a new user data stream.
    /// The stream will close after 60 minutes unless a keepalive is sent.
    ///
    /// Weight: 1
    pub async fn user_data_stream(&self) -> LibResult<ListenKey> {
        self.client
            .post(V1_USER_DATA_STREAM)?
            .auth_header()?
            .send()
            .await
    }
}
