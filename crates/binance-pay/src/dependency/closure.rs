use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::any::Any;

use futures::channel::oneshot;
use erased_serde::Serialize;

use crate::Time;

pub type DependencyError = String;
pub type DependencyResult<T> = std::result::Result<T, DependencyError>;
pub type SignJson = dyn Serialize;
pub type SignSender = oneshot::Sender<DependencyResult<String>>;

pub struct SignData<J: Serialize + ?Sized> {
    pub time: Time,
    pub nonce: String,
    pub json: Box<J>,
}

pub struct SignClosure<J: ?Sized, CB, F>
where
    J: Serialize + 'static,
    CB: Fn(SignData<J>, SignSender) -> F + Clone,
    F: Future<Output = ()>,
{
    cb: Box<CB>,
    _marker: PhantomData<J>,
}

impl<J, CB, F> Clone for SignClosure<J, CB, F>
where
    J: Serialize + 'static,
    CB: Fn(SignData<J>, SignSender) -> F + Clone,
    F: Future<Output = ()>,
{
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
            _marker: PhantomData,
        }
    }
}

impl<J, CB, F> SignClosure<J, CB, F>
where
    J: Serialize + ?Sized,
    CB: Unpin + 'static,
    CB: Fn(SignData<J>, SignSender) -> F + Clone,
    F: Future<Output = ()> + 'static,
{
    pub fn new(cb: CB) -> Self {
        Self {
            cb: Box::new(cb),
            _marker: PhantomData,
        }
    }

    pub fn call_inner(self, data: SignData<J>, sender: SignSender) -> F {
        (self.cb)(data, sender)
    }
}

struct Json<J: Serialize> {
    value: J,
}

// trait SerValue {
//     fn value(self) -> impl Serialize;
// }


pub struct Data {
    pub time: Time,
    pub nonce: String,
    pub json: Box<dyn Serialize + Sync + Send>,
}

unsafe impl Send for Data {}

impl Data {
    pub fn value(self) -> impl serde::Serialize + Sync + Send {
        self.json
    }
}

pub trait AppSign: Any {
    fn box_clone(&self) -> Box<dyn AppSign + Send>;

    fn clone_as_any(&self) -> Box<dyn Any + Send>;
}

impl<T: Any + Sync + Send + Clone> AppSign for T {
    fn box_clone(&self) -> Box<dyn AppSign + Send> {
        Box::new(self.clone())
    }

    fn clone_as_any(&self) -> Box<dyn Any + Send> {
        Box::new(self.clone())
    }
}

type FnResult = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
type FnClosure = Box<dyn Fn(Box<dyn AppSign + Send>, Data, SignSender) -> FnResult + Send + Sync>;
// type FnClosure = Box<dyn Fn(Box<Any + Send>, Data, SignSender) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>;


#[derive(Clone)]
pub struct Closure {
    inner: Arc<Inner>
}

struct Inner {
    app: Box<dyn AppSign + Send>,
    closure: FnClosure,
}

impl Inner {

    fn sign(&self, data: Data, tx: SignSender) -> FnResult {
        (self.closure)(self.app.box_clone(), data, tx)
    }
}

impl Closure {
    pub fn new(app: Box<dyn AppSign + Send>, closure: FnClosure) -> Self {
        Self {
            inner: Arc::new(Inner { app, closure })
        }
    }

    pub fn sign<J>(&self, time: Time, nonce: String, json: J, tx: SignSender) -> FnResult where J: Serialize + Sync + Send + 'static {
        let data = Data {
            time,
            nonce,
            json: Box::new(json)
        };
        let inner = Arc::clone(&self.inner);
        (*inner).sign(data, tx)
        // unimplemented!()
    }
}