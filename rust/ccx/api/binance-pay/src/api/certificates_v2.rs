use crate::api::Api;
use crate::error::LibResult;
use crate::types::certificate::Certificate;
use crate::types::enums::StatusRequest;
use crate::Time;

const BINANCEPAY_OPENAPI_CERTIFICATES: &str = "/binancepay/openapi/certificates";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2CertificateRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2CertificateResponse {
    pub status: StatusRequest,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Certificate>>,
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v2_certificates(
        &self,
        request: V2CertificateRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<V2CertificateResponse> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_CERTIFICATES, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}
