use std::future::Future;
use std::pin::Pin;

use crate::client::Nonce;
use crate::KrakenError;
use crate::KrakenResult;
use ccx_api_lib::ApiCred;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = KrakenResult<String>> + Send + 'a>>;

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignKraken>;
}

pub trait SignKraken: SignerClone + Sync + Send {
    fn sign<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        method: &'b str,
        query: &'b str,
    ) -> SignResult<'a>;

    fn key(&self) -> &str;
}

impl<T> SignerClone for T
where
    T: SignKraken + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn SignKraken> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignKraken> {
    fn clone(&self) -> Box<dyn SignKraken> {
        self.clone_box()
    }
}

impl SignKraken for ApiCred {
    fn sign<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        method: &'b str,
        query: &'b str,
    ) -> SignResult<'a> {
        Box::pin(async move {
            let f = async move {
                let decoded_secret = base64::decode(&self.secret).map_err(|e| {
                    KrakenError::other(format!("Failed to deserialize key: {:?}", e))
                })?;
                let signature = sign(method, nonce, query, &decoded_secret);
                Ok(signature)
            };
            let res: KrakenResult<String> = f.await;
            res
        })
    }

    fn key(&self) -> &str {
        self.key.as_str()
    }
}

fn sign(path: &str, nonce: Nonce, body: &str, decoded_secret: &[u8]) -> String {
    use hmac::Hmac;
    use hmac::Mac;
    use hmac::NewMac;
    use sha2::Digest;
    use sha2::Sha256;
    use sha2::Sha512;

    // let payload = serde_urlencoded::to_string(payload).expect("serialize payload");
    let mut m256 = Sha256::new();
    m256.update(nonce.decimal());
    m256.update(body);
    let payload = m256.finalize();

    let mut m512 =
        Hmac::<Sha512>::new_varkey(decoded_secret).expect("HMAC can take key of any size");
    m512.update(path.as_bytes());
    m512.update(&payload);

    let res = m512.finalize().into_bytes();
    base64::encode(&res)
}

pub trait KrakenSigner {
    type Signer: SignKraken;

    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        method: &'b str,
        query: &'b str,
    ) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl<T: SignKraken> KrakenSigner for T {
    type Signer = T;

    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        method: &'b str,
        query: &'b str,
    ) -> SignResult<'a> {
        self.sign(nonce, method, query)
    }

    fn api_key(&self) -> &str {
        self.key()
    }
}
