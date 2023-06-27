use core::fmt;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EitherOrderId {
    Bitstamp(u64),
    Client(Uuid),
}

impl fmt::Display for EitherOrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EitherOrderId::Bitstamp(order_id) => write!(f, "{order_id}"),
            EitherOrderId::Client(order_id) => write!(f, "client:{order_id}"),
        }
    }
}
