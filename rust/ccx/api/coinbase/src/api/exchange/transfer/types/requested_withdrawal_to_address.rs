use crate::api::exchange::prelude::*;

/// Represents a requested withdraw to address object from Coinbase Exchange/Pro API.
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedWithdrawal {
    /// The unique identifier of the requested withdraw to address.
    pub id: Uuid,
    /// The amount of the requested withdraw to address, as a decimal.
    pub amount: Decimal,
    /// The currency of the requested withdraw to address.
    pub currency: Atom,
    /// The time at which the withdraw to address is scheduled to be paid out.
    pub payout_at: DtCoinbase,
    /// The fee associated with the requested withdraw to address.
    pub fee: Decimal,
    /// The subtotal amount of the requested withdraw to address.
    pub subtotal: Decimal,
}
