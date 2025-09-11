use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;

use crate::KrakenError;
use crate::KrakenResult;
use crate::client::Nonce;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = KrakenResult<String>> + Send + 'a>>;

pub trait KrakenSigner: Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        method: &'b str,
        query: &'b str,
    ) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl KrakenSigner for ApiCred {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        method: &'b str,
        query: &'b str,
    ) -> SignResult<'a> {
        use base64::Engine as _;
        use base64::engine::general_purpose;

        Box::pin(async move {
            let decoded_secret = general_purpose::STANDARD
                .decode(&self.secret)
                .map_err(|e| KrakenError::other(format!("Failed to deserialize key: {:?}", e)))?;
            Ok(sign(method, nonce, query, &decoded_secret))
        })
    }

    fn api_key(&self) -> &str {
        self.key.as_str()
    }
}

fn sign(path: &str, nonce: Nonce, body: &str, decoded_secret: &[u8]) -> String {
    use base64::Engine as _;
    use base64::engine::general_purpose;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Digest;
    use sha2::Sha256;
    use sha2::Sha512;

    // let payload = serde_urlencoded::to_string(payload).expect("serialize payload");
    let mut m256 = Sha256::new();
    m256.update(nonce.decimal());
    m256.update(body);
    let payload = m256.finalize();

    let mut m512 =
        Hmac::<Sha512>::new_from_slice(decoded_secret).expect("HMAC can take key of any size");
    m512.update(path.as_bytes());
    m512.update(&payload);

    let res = m512.finalize().into_bytes();
    general_purpose::STANDARD.encode(res)
}
