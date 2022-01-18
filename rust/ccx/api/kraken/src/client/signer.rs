use std::future::Future;
use std::pin::Pin;

use erased_serde::Serialize;

use crate::KrakenResult;

pub enum Query<'a> {
    Url(&'a str),
    Params(&'a dyn Serialize),
}

unsafe impl<'a> Send for Query<'a> {}

pub struct Data<'a> {
    pub nonce: u64,
    pub method: &'a str,
    pub params: Query<'a>,
}

unsafe impl<'a> Send for Data<'a> {}

pub type SignResult<'a> = Pin<Box<dyn Future<Output = KrakenResult<String>> + Send + 'a>>;

pub trait SignerClone {
    fn clone_box(&self) -> Box<dyn SignKraken>;
}

pub trait SignKraken: SignerClone + Sync + Send {
    fn sign<'a, 'b:'a, 'c:'b>(&'c self, data: Data<'b>) -> SignResult<'a>;
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
