mod balances;
mod deposit_address;
mod deposits;
mod transfer;
mod withdrawal_history;

pub use balances::*;
pub use deposit_address::*;
pub use deposits::*;
pub use transfer::*;
pub use withdrawal_history::*;

use super::GateApi;

/// Spot trading
#[derive(ref_cast::RefCast, Clone)]
#[repr(transparent)]
pub struct WalletApi<S>(GateApi<S>);
