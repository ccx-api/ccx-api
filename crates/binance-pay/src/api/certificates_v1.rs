use crate::BinancePayResponse;
use crate::Time;
use crate::api::Api;
use crate::error::LibResult;
use crate::types::certificate::Certificate;

const BINANCEPAY_OPENAPI_CERTIFICATES: &str = "/binancepay/openapi/certificates";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1CertificateRequest {
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, // long     Y   -   The merchant account id, issued when merchant been created at Binance.
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v1_certificates(
        &self,
        request: V1CertificateRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<BinancePayResponse<Vec<Certificate>>> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_CERTIFICATES, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}
