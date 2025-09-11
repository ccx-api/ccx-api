use crate::BinancePayResponse;
use crate::Time;
use crate::api::Api;
use crate::error::LibResult;
use crate::types::certificate::Certificate;

const BINANCEPAY_OPENAPI_CERTIFICATES: &str = "/binancepay/openapi/certificates";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2CertificateRequest;

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v2_certificates(
        &self,
        request: V2CertificateRequest,
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
