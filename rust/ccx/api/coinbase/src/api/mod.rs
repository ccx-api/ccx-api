pub mod exchange;
pub mod prime;
pub mod trade;

mod prelude {
    pub use rust_decimal::prelude::Zero;
    pub use rust_decimal::Decimal;
    pub use serde::Deserialize;
    pub use serde::Serialize;
    pub use serde_repr::Deserialize_repr;
    pub use serde_repr::Serialize_repr;
    pub use uuid::Uuid;

    pub use crate::client::Task;
    // pub use crate::client::Nonce;
    pub use crate::error::*;
    pub use crate::proto::*;
    pub use crate::Atom;
}
