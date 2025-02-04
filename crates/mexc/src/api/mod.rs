pub mod spot;

mod prelude {
    pub use std::collections::HashMap;

    pub use rust_decimal::prelude::Zero;
    pub use rust_decimal::Decimal;
    pub use serde::Deserialize;
    pub use serde::Serialize;
    pub use serde_repr::Deserialize_repr;
    pub use serde_repr::Serialize_repr;

    pub use crate::error::*;
    pub use crate::proto::*;
    pub use crate::Atom;
    pub use crate::TimeWindow;
}
