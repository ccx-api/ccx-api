use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;
use erased_serde::Serialize;

use crate::error::LibResult;

pub type SignParams = dyn Serialize + Sync + Send;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = LibResult<String>> + Send + 'a>>;

pub trait BinancePaySigner: Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        time: i64,
        nonce: &'b str,
        params: &'b SignParams,
    ) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl BinancePaySigner for ApiCred {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        time: i64,
        nonce: &'b str,
        params: &'b SignParams,
    ) -> SignResult<'a> {
        Box::pin(async move {
            let json = serde_json::to_string(params)?;
            let payload = format!("{}\n{}\n{}\n", time, nonce, json);
            Ok(sign(&payload, self.secret.as_ref()))
        })
    }

    fn api_key(&self) -> &str {
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
