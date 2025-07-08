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
use crate::types::WsPosition;
use crate::types::WsSettlementOrder;

pub const API_POSITIONS: &str = "api/positions";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PositionsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PositionsResponse {
    /// 0
    /// Efx::DealId
    /// Next Deal or Settlement Id
    pub id: DealId,
    /// 1
    /// Array of Positions
    /// Asset Positions
    pub positions: Vec<Position>,
    /// 2
    /// Array of Orders
    /// Active Orders (for makers only)
    pub orders: Vec<Order>,
    /// 3
    /// Array of SettlementOrders
    /// Active Settlement Orders
    pub settlement_orders: Vec<SettlementOrder>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct Position {
    /// 0
    /// string
    /// Currency name
    // #[index = 0]
    pub currency_name: String,
    /// 1
    /// Efx::Size
    /// Value
    // #[index = 1]
    pub value: Size,
    /// 2
    /// Efx::ClientId
    /// Counterparty Id
    // #[index = 2]
    pub counterparty_id: ClientId,
    /// 3
    /// Efx::Size
    /// Max reachable position
    // #[index = 3]
    pub max_reachable_position: Size,
    /// 4
    /// Efx::Size
    /// Min reachable position
    // #[index = 4]
    pub min_reachable_position: Size,
}

impl From<WsPosition> for Position {
    fn from(pos: WsPosition) -> Self {
        Self {
            currency_name: pos.currency_name,
            value: pos.value,
            counterparty_id: pos.counterparty_id,
            max_reachable_position: pos.max_reachable_position,
            min_reachable_position: pos.min_reachable_position,
        }
    }
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct Order {
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
    pub r#type: OrderTypeByRepr,
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
    /// Order Id
    pub id: OrderId,
    /// 5
    /// Efx::ClientOrderId
    /// Client Order Id
    pub client_order_id: ClientOrderId,
    /// 6
    /// Efx::Price
    /// Order price
    pub price: Price,
    /// 7
    /// Efx::Size
    /// Order Initial Size Or Volume (Depending on Order was initially created by size or volume)
    pub initial_size: Size,
    /// 8
    /// Efx::Size
    /// Remaining Order Size Or Volume (Depending on Order was initially created by size or volume)
    pub remaining_size: Size,
    /// 9
    /// Efx::Timestamp
    /// Created At
    pub created_at: Timestamp,
    /// 10
    /// unsigned int16
    /// If order was created by size or by volume
    /// 0 - by size
    /// 1 - by volume
    pub create_type: OrderCreateType,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, Eq, PartialEq)]
pub struct SettlementOrder {
    /// 0
    /// Efx::OrderId
    /// Settlement order id
    pub id: OrderId,
    /// 1
    /// string
    /// Currency 1
    pub currency1: String,
    /// 2
    /// string
    /// Currency 2
    pub currency2: String,
    /// 3
    /// Efx::Size
    /// Size 1
    pub size1: Size,
    /// 4
    /// Efx::Size
    /// Size 2
    pub size2: Size,
    /// 5
    /// Efx::Timestamp
    /// Created At
    pub created_at: Timestamp,
    /// 6
    /// Efx::ClientId
    /// Counterparty id
    pub counterparty_id: ClientId,
    /// 7
    /// Efx::Network name 1
    /// Network name 1
    pub network_name_1: String,
    /// 8
    /// Efx::Network name 2
    /// Network name 2
    pub network_name_2: String,
}

impl From<WsSettlementOrder> for SettlementOrder {
    fn from(so: WsSettlementOrder) -> Self {
        Self {
            id: so.settlement_order_id,
            currency1: so.currency1,
            currency2: so.currency2,
            size1: so.size1,
            size2: so.size2,
            created_at: so.created_at,
            counterparty_id: so.counterparty_id,
            network_name_1: so.network_name_1,
            network_name_2: so.network_name_2,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_positions() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<PositionsRequest>(json);

        let json = r#"[
            5,
            [
                [ 
                    "BTC",
                    -10000000,
                    2,
                    0,
                    -10000000
                ],
                [
                    "USD",
                    100000000000,
                    2,
                    100000000000,
                    0
                ]
            ],
            [
                [
                    "BTC-USD",
                    0,
                    0,
                    0,
                    1234,
                    0,
                    999900000000,
                    10000000,
                    8000000,
                    1558051200000,
                    0
                ]
            ],
            [
                [
                    1229,
                    "BTC",
                    "USD",
                    10000000,
                    100000000000,
                    1558050900000,
                    2,
                    "BTC",
                    ""
                ]
            ]
        ]"#;
        test_serde_response::<PositionsResponse>(json);
        test_serde_response_err::<PositionsResponse>();
    }
}
