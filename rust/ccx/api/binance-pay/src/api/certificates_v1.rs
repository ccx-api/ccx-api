use crate::api::Api;
use crate::error::LibResult;
use crate::types::certificate::Certificate;
use crate::types::enums::StatusRequest;
use crate::Time;

const BINANCEPAY_OPENAPI_CERTIFICATES: &str = "/binancepay/openapi/certificates";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1CertificateRequest {
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, // long     Y   -   The merchant account id, issued when merchant been created at Binance.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1CertificateResponse {
    pub status: StatusRequest, // string               Y   -   "SUCCESS" or "FAIL"	status of the API request
    pub code: String,          // string               Y	-	request result code, refer to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Certificate>>, // Array of Certificate Y	-	response body, refer to
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>, // string               Y	-
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v1_certificates(
        &self,
        request: V1CertificateRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<V1CertificateResponse> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_CERTIFICATES, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}
