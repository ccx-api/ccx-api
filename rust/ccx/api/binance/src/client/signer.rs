use std::future::Future;
use std::pin::Pin;

use ccx_api_lib::ApiCred;

use crate::BinanceResult;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = BinanceResult<String>> + Send + 'a>>;

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

// pub type DynSigner = std::sync::Arc<dyn BinanceSigner>;

impl SignBinance for std::sync::Arc<dyn BinanceSigner> {
    fn sign<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a> {
        self.as_ref().sign_data(query)
    }

    fn key(&self) -> &str {
        self.as_ref().api_key()
    }
}

pub trait BinanceSigner: SignerClone + Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}

impl<T: SignBinance + Clone + 'static> BinanceSigner for T {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(&'c self, query: &'b str) -> SignResult<'a> {
        self.sign(query)
    }

    fn api_key(&self) -> &str {
        self.key()
    }
}

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn BinanceSigner>;

    fn clone_arc(&self) -> std::sync::Arc<dyn BinanceSigner>;
}

impl<T> SignerClone for T
where
    T: BinanceSigner + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn BinanceSigner> {
        Box::new(self.clone())
    }

    fn clone_arc(&self) -> std::sync::Arc<dyn BinanceSigner> {
        std::sync::Arc::new(self.clone())
    }
}

// impl Clone for Box<dyn BinanceSigner> {
//     fn clone(&self) -> Box<dyn BinanceSigner> {
//         self.clone_box()
//     }
// }
