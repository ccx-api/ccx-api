use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum DepositStatus {
    Pending = 0,
    Success = 1,
    Rejected = 2,
    Processing = 6,
    WrongDeposit = 7,
    WaitingForConfirmation = 8,
}
