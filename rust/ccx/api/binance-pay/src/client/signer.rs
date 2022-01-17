use std::future::Future;
use std::pin::Pin;

use erased_serde::Serialize;

use crate::error::LibResult;

pub type SignParams = dyn Serialize + Sync + Send;

#[allow(dead_code)]
pub type SignResult = Pin<Box<dyn Future<Output = LibResult<String>> + Send>>;

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignBinancePay>;
}

pub trait SignBinancePay: SignerClone + Sync + Send {
    fn sign(&self, time: i64, nonce: &str, params: Box<SignParams>) -> SignResult;
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
