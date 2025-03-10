use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u8)]
pub enum DepositStatus {
    Small = 1,
    TimeDelay = 2,
    LargeDelay = 3,
    Pending = 4,
    Success = 5,
    Auditing = 6,
    Rejected = 7,
}
