use std::future::Future;
use std::pin::Pin;

use erased_serde::Serialize;

use crate::error::LibResult;

pub type SignParams = dyn Serialize + Sync + Send;

pub type SignResult<'a> = Pin<Box<dyn Future<Output = LibResult<String>> + Send + 'a>>;

pub struct Data<'a> {
    pub time: i64,
    pub nonce: &'a str,
    pub params: &'a SignParams,
}

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignBinancePay>;
}

pub trait SignBinancePay: SignerClone + Sync + Send {
    fn sign<'a, 'b:'a, 'c:'b>(&'c self, data: Data<'b>) -> SignResult<'a>;
}

impl<T> SignerClone for T
where
    T: SignBinancePay + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn SignBinancePay> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignBinancePay> {
    fn clone(&self) -> Box<dyn SignBinancePay> {
        self.clone_box()
    }
}
