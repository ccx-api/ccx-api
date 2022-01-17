use std::future::Future;
use std::pin::Pin;

use erased_serde::Serialize;

use crate::BinanceResult;

pub enum Query {
    Url(String),
    Params(Box<dyn Serialize>),
}

#[allow(dead_code)]
type SignResult = Pin<Box<dyn Future<Output = BinanceResult<String>> + Send>>;

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignBinance>;
}

pub trait SignBinance: SignerClone + Sync + Send {
    fn sign(&self, query: Query) -> SignResult;
}

impl<T> SignerClone for T
where
    T: SignBinance + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn SignBinance> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignBinance> {
    fn clone(&self) -> Box<dyn SignBinance> {
        self.clone_box()
    }
}
