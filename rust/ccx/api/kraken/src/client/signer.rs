use std::future::Future;
use std::pin::Pin;

use erased_serde::Serialize;

use crate::KrakenResult;

pub enum Query {
    Url(String),
    Params(Box<dyn Serialize>),
}

#[allow(dead_code)]
type SignResult = Pin<Box<dyn Future<Output = KrakenResult<String>>>>;

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignKraken>;
}

pub trait SignKraken: SignerClone + Sync + Send {
    fn sign(&self, nonce: u64, method: String, query: Query) -> SignResult;
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
