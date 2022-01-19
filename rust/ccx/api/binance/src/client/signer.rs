use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;

use crate::BinanceResult;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = BinanceResult<String>> + Send + 'a>>;

// pub trait SignerClone {
//     fn clone_box(&self) -> Box<dyn SignBinance>;
// }

pub trait SignBinance: Sync + Send {
    fn sign<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a>;

    fn key(&self) -> &str;
}

impl SignBinance for ApiCred {
    fn sign<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a> {
        Box::pin(async move {
            let f = async move {
                let secret = self.secret.as_bytes().to_vec();

                let signature = sign(query, &secret);
                Ok(signature)
            };
            let res: BinanceResult<String> = f.await;
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
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_varkey(secret).expect("HMAC can take key of any size");
    mac.update(query.as_bytes());

    let res = mac.finalize().into_bytes();
    format!("{:x}", res)
}

pub trait BinaneSigner {
    type Signer: SignBinance;

    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl<T: SignBinance> BinaneSigner for T {
    type Signer = T;

    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a> {
        self.sign(query)
    }

    fn api_key(&self) -> &str {
        self.key()
    }
}
