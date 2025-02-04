mod balances;
mod transfer;
mod withdrawal_history;

pub use balances::*;
use ref_cast::RefCast;
pub use transfer::*;
pub use withdrawal_history::*;

use super::GateApi;

/// Spot trading
#[derive(RefCast, Clone)]
#[repr(transparent)]
pub struct WalletApi<S>(GateApi<S>);
