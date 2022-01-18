use std::future::Future;
use std::pin::Pin;

use erased_serde::Serialize;

use crate::BinanceResult;

pub enum Query<'a> {
    Url(&'a str),
    Params(&'a dyn Serialize),
}

unsafe impl<'a> Send for Query<'a> {}

pub type SignResult<'a> = Pin<Box<dyn Future<Output = BinanceResult<String>> + Send + 'a>>;

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignBinance>;
}

pub trait SignBinance: SignerClone + Sync + Send {
    fn sign<'a, 'b:'a, 'c:'b>(&'c self, query: Query<'b>) -> SignResult<'a>;
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
