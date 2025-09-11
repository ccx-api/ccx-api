pub mod spot;
// TODO pub mod futures;

mod prelude {
    pub use rust_decimal::Decimal;
    pub use rust_decimal::prelude::Zero;
    pub use serde::Deserialize;
    pub use serde::Serialize;
    pub use serde_repr::Deserialize_repr;
    pub use serde_repr::Serialize_repr;

    // pub use crate::proto::*;
    pub use crate::Atom;
    pub use crate::client::Nonce;
    pub use crate::error::*;
}
