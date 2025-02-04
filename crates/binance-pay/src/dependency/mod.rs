mod closure;

pub use closure::DependencyResult;
// pub use closure::Sign;
pub use closure::SignJson;
pub use closure::SignClosure;
pub use closure::SignData;
pub use closure::Closure;
pub use closure::Data;
pub use closure::AppSign;

// pub trait Sign {
//     fn sign(
//         &self,
//         tx: futures::channel::oneshot::Sender<DependencyResult<String>>,
//     ) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
// }

use crate::Time;
use std::pin::Pin;
use std::future::Future;
use closure::SignSender;
pub trait Sign {
    type Json: ?Sized;

    fn sign(
        &self,
        time: &Time,
        nonce: &str,
        // json: Box<Self::Json>,
        json: &Self::Json,
        tx: SignSender,
    ) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}