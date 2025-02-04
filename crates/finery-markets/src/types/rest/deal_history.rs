use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

use crate::types::CancelReason;
use crate::types::ClientId;
use crate::types::ClientOrderId;
use crate::types::DealId;
use crate::types::OrderCreateType;
use crate::types::OrderId;
use crate::types::OrderTypeByRepr;
use crate::types::Pair;
use crate::types::Price;
use crate::types::SideByRepr;
use crate::types::Size;
use crate::types::Timestamp;

pub const API_DEAL_HISTORY: &str = "api/dealHistory";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DealHistoryRequest {
    /// instrument
    /// string
    /// Instrument name.
    pub instrument: Pair,
    /// If specified only return deals for this instrument
    /// till
    /// Efx::DealId
    /// If specified only return deals with lesser ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub till: Option<DealId>,
    /// from
    /// Efx::Timestamp
    /// If specified only return deals with equal or greater timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Timestamp>,
    /// to
    /// Efx::Timestamp
    /// If specified only return deals with lesser timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Timestamp>,
    /// limit
    /// unsigned int16
    /// Default: 250
    /// Maximum number of deals to return (capped at 250)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DealHistoryResponse(pub Vec<Deal>);

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct Deal {
    /// 0
    /// string
    /// Instrument name
    pub instrument_name: Pair,
    /// 1
    /// unsigned int16
    /// Order Type
    /// 0 - limit
    /// 1 - post only
    /// 2 - limit IOC
    /// 3 - limit FOK
    /// 4 - market IOC
    /// 5 - market FOK
    pub order_type: OrderTypeByRepr,
    /// 2
    /// Efx::Side
    /// SideByRepr
    /// 0 - bid
    /// 1 - ask
    pub side: SideByRepr,
    /// 3
    /// unsigned int16
    /// Cancel reason
    /// 0 - in place or filled
    /// 1 - by client
    /// 2 - as non-book order
    /// 3 - by self-trade prevention
    /// 4 - cancel-on-disconnect
    pub cancel_reason: CancelReason,
    /// 4
    /// Efx::OrderId
    /// Order id
    pub order_id: OrderId,
    /// 5
    /// Efx::ClientOrderId
    /// Client order id
    pub client_order_id: ClientOrderId,
    /// 6
    /// Efx::Price
    /// Order price
    pub order_price: Price,
    /// 7
    /// Efx::Size
    /// Order Initial Size Or Volume (depending on whether order was initiated by volume)
    pub initial_size: Size,
    /// 8
    /// Efx::Size
    /// Remaining Order Size Or Volume after deal (depending on whether order was initiated by volume)
    pub remaining_size: Size,
    /// 9
    /// Efx::Timestamp
    /// Order Created At
    pub created_at: Timestamp,
    /// 10
    /// Efx::Timestamp
    /// Deal Moment
    pub deal_moment: Timestamp,
    /// 11
    /// Efx::DealId
    /// Deal id
    pub deal_id: DealId,
    /// 12
    /// Efx::Side
    /// Deal aggressor side
    pub deal_aggressor_side: SideByRepr,
    /// 0 - bid
    /// 1 - ask
    /// 13
    /// Efx::Price
    /// Deal price
    pub deal_price: Price,
    /// 14
    /// Efx::Size
    /// Deal size
    pub deal_size: Size,
    /// 15
    /// Efx::Size
    /// Deal volume
    pub deal_volume: Size,
    /// 16
    /// Efx::Size
    /// Deal delta in quote (balance) currency
    pub deal_delta: Size,
    /// 17
    /// Efx::ClientId
    /// Counterparty id
    pub counterparty_id: ClientId,
    /// 18
    /// unsigned int16
    /// If order was created by size or by volume
    /// 0 - by size
    /// 1 - by volume
    pub create_type: OrderCreateType,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_deal_history() {
        let json = r#"
        {
            "instrument": "BTC-USD"
        }
        "#;
        test_serde_value_type::<DealHistoryRequest>(json);

        let json = r#"
        [
            [
                "BTC-USD",
                0,
                0,
                0,
                1234,
                0,
                9900000000,
                10000000,
                9998000,
                1558051200000,
                1558052600000,
                12,
                1,
                9900000000,
                2000,
                19800000000000,
                100000,
                0,
                1
            ]
        ]
        "#;
        test_serde_response::<DealHistoryResponse>(json);
        test_serde_response_err::<DealHistoryResponse>();
    }
}
