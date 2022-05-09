use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;

use crate::BinanceResult;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = BinanceResult<String>> + Send + 'a>>;

pub trait BinanceSigner: Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl BinanceSigner for ApiCred {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a> {
        Box::pin(async move {
            Ok(sign(query, self.secret.as_bytes()))
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
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_varkey(secret).expect("HMAC can take key of any size");
    mac.update(query.as_bytes());

    let res = mac.finalize().into_bytes();
    format!("{:x}", res)
}

