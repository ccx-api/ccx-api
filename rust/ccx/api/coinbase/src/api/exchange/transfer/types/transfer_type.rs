use crate::api::exchange::prelude::*;

/// An enumeration of possible transfer types.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
    Deposit,
    Withdraw,
    InternalDeposit,
    InternalWithdraw,
}
