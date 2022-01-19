use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;
use erased_serde::Serialize;

use crate::error::LibResult;

pub type SignParams = dyn Serialize + Sync + Send;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = LibResult<String>> + Send + 'a>>;

pub trait SignBinancePay: Sync + Send {
    fn sign<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        time: i64,
        nonce: &'b str,
        params: &'b SignParams,
    ) -> SignResult<'a>;

    fn key(&self) -> &str;
}

impl SignBinancePay for ApiCred {
    fn sign<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        time: i64,
        nonce: &'b str,
        params: &'b SignParams,
    ) -> SignResult<'a> {
        Box::pin(async move {
            let f = async move {
                let json = serde_json::to_string(params)?;
                let payload = format!("{}\n{}\n{}\n", time, nonce, json);
                let signature = sign(&payload, self.secret.as_ref());
                Ok(signature)
            };
            let res: LibResult<String> = f.await;
            res
        })
    }

    fn key(&self) -> &str {
        self.key.as_str()
    }
}

fn sign(query: &str, secret: &[u8]) -> String {
    use hmac::Hmac;
    use hmac::Mac;
    use hmac::NewMac;
    use sha2::Sha512;

    log::debug!("sign query  :: {}", query);
    let mut mac = Hmac::<Sha512>::new_varkey(secret).expect("HMAC can take key of any size");
    mac.update(query.as_bytes());

    let res = mac.finalize().into_bytes();
    hex::encode(res)
}

pub trait BinancePaySigner {
    type Signer: SignBinancePay;

    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        time: i64,
        nonce: &'b str,
        params: &'b SignParams,
    ) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl<T: SignBinancePay> BinancePaySigner for T {
    type Signer = T;

    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        time: i64,
        nonce: &'b str,
        params: &'b SignParams,
    ) -> SignResult<'a> {
        self.sign(time, nonce, params)
    }

    fn api_key(&self) -> &str {
        self.key()
    }
}
