use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;

use crate::error::LibResult;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = LibResult<String>> + Send + 'a>>;

pub trait FinerySigner: Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, content: &'b str) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl FinerySigner for ApiCred {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, content: &'b str) -> SignResult<'a> {
        Box::pin(async move { Ok(sign(content, self.secret.as_ref())) })
    }

    fn api_key(&self) -> &str {
        self.key.as_str()
    }
}

fn sign(content: &str, secret: &[u8]) -> String {
    use base64::{engine::general_purpose, Engine as _};
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Sha384;

    log::debug!("sign query :: {}", content);
    let mut mac = Hmac::<Sha384>::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(content.as_bytes());

    let res = mac.finalize().into_bytes();
    general_purpose::STANDARD.encode(res)
}
