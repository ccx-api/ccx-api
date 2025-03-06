use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum TransferType {
    External = 0,
    Internal = 1,
}
