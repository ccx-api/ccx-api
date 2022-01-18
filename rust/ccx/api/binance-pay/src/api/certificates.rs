use crate::api::Api;
use crate::error::LibResult;
use crate::types::enums::StatusRequest;
use crate::Time;

const BINANCEPAY_OPENAPI_CERTIFICATES: &str = "/binancepay/openapi/certificates";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificateRequest {
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, // long     Y   -   The merchant account id, issued when merchant been created at Binance.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certificate {
    #[serde(rename = "certSerial")]
    pub cert_serial: String, //  string	Y	-	public key hash value
    #[serde(rename = "certPublic")]
    pub cert_public: String, //  string	Y	-	public key
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificateResponse {
    pub status: StatusRequest, // string               Y   -   "SUCCESS" or "FAIL"	status of the API request
    pub code: String,          // string               Y	-	request result code, refer to
    pub data: Option<Vec<Certificate>>, // Array of Certificate Y	-	response body, refer to
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>, // string               Y	-
}

impl<S: crate::client::BinanePaySigner> Api<S> {
    pub async fn certificates(
        &self,
        request: CertificateRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<CertificateResponse> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_CERTIFICATES, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}
