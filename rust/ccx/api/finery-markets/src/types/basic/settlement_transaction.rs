use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::ClientId;
use crate::types::DealId;
use crate::types::OrderId;
use crate::types::Size;
use crate::types::Timestamp;

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct SettlementTransaction {
    /// 0
    /// Efx::ClientId
    /// Counterparty id
    pub counterparty_id: ClientId,
    /// 1
    /// string
    /// Currency name
    pub currency_name: String,
    /// 2
    /// Efx::Size
    /// Amount
    pub amount: Size,
    /// 3
    /// Efx::OrderId
    /// Settlement order id
    pub settlement_order_id: OrderId,
    /// 4
    /// string
    /// Comment
    pub comment: String,
    /// 5
    /// Efx::Timestamp
    /// Created at
    pub created_at: Timestamp,
    /// 6
    /// string
    /// Tx id
    pub tx_id: String,
    /// 7
    /// Efx::Timestamp
    /// Sent at
    pub sent_at: Timestamp,
    /// 8
    /// unsigned int32
    /// Reserved flags
    pub reserved_flags: u32,
    /// 9
    /// Efx::Timestamp
    /// Transaction moment
    pub transaction_moment: Timestamp,
    /// 10
    /// Efx::DealId
    /// Transaction id
    pub transaction_id: DealId,
    /// 11
    /// Efx::Size
    /// Network fee
    pub network_fee: Size,
}

impl SettlementTransaction {
    pub fn fee(&self) -> Option<Size> {
        match self.network_fee {
            0 => None,
            fee => Some(fee),
        }
    }

    pub fn dt(&self) -> Timestamp {
        if self.transaction_moment > 0 {
            return self.transaction_moment;
        }
        if self.sent_at > 0 {
            return self.sent_at;
        }
        self.created_at
    }
}
