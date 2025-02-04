mod withdraw;

use ref_cast::RefCast;
pub use withdraw::*;

use super::GateApi;

/// Withdrawal operations
#[derive(RefCast, Clone)]
#[repr(transparent)]
pub struct WithdrawalApi<S>(GateApi<S>);
