pub const API_ADD: &str = "api/add";

use crate::types::CancelReason;
use crate::types::ClientOrderId;
use crate::types::DealId;
use crate::types::OrderId;
use crate::types::OrderTypeByName;
use crate::types::Pair;
use crate::types::Price;
use crate::types::SideByName;
use crate::types::Size;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddRequest {
    /// instrument
    /// string
    /// Instrument name
    pub instrument: Pair,
    /// clientOrderId
    /// Efx::ClientOrderId
    /// User data attached to the order
    #[serde(rename = "clientOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
    /// price
    /// Efx::Price
    /// Order limit price (for postOnly and limit orders)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    /// size
    /// Efx::Size
    /// Order size. If specified - volume shouldn't be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    /// volume
    /// Efx::Size
    /// Order volume to be filled. Can be used by takers only. If specified - size shouldn't be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<Size>,
    /// side
    /// string
    /// Order side, use "bid" to buy and "ask" to sell
    pub side: SideByName,
    /// type
    /// string
    /// Order types "limitIOC", "limitFOK", "marketIOC", "marketFOK" can only be used by market takers. "postOnly" and "limit" can only be used by market makers.
    #[serde(rename = "type")]
    pub r#type: OrderTypeByName,
    /// cod
    /// boolean
    /// Cancel On Disconnect flag (will be used only within Web Socket authenticated connection)
    pub cod: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddResponse {
    /// id
    /// Efx:OrderId
    /// New Order Id
    pub id: OrderId,
    /// clientOrderId
    /// Efx::ClientOrderId
    /// Optional user data attached to the order
    #[serde(rename = "clientOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
    /// remainingSize
    /// Efx::Size
    /// Remaining Order size after aggressive deals (if order was placed by size)
    #[serde(rename = "remainingSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<Size>,
    /// remainingVolume
    /// Efx::Size
    /// Remaining Order size after aggressive deals (if order was placed by volume)
    #[serde(rename = "remainingVolume")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_volume: Option<Size>,
    /// cancelReason
    /// unsigned int16
    /// Cancel Reason:
    /// 0 - in-place/filled
    /// 1 - by client
    /// 2 - as non-book order
    /// 3 - by self-trade prevention
    /// 4- on disconnect)
    #[serde(rename = "cancelReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<CancelReason>,
    /// deals
    /// array of objects
    /// Initial (taker) deals
    pub deals: Vec<Deal>,
}

impl AddResponse {
    pub fn cancel_reason(&self) -> CancelReason {
        self.cancel_reason.unwrap_or_default()
    }

    pub fn canceled(&self) -> bool {
        self.cancel_reason() != CancelReason::InPlaceOrFilled
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Deal {
    /// id
    /// Efx::DealId
    /// Deal Id
    pub id: DealId,
    /// price
    /// Efx::Price
    /// Deal price
    pub price: Price,
    /// size
    /// Efx::Size
    /// Deal size
    pub size: Size,
    /// volume
    /// Efx::Size
    /// Deal volume
    pub volume: Size,
    /// delta
    /// Efx::Size
    /// Deal delta in quote (balance) currency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<Size>,
    /// counterpartyId
    /// unsigned int64
    /// Counterparty id
    #[serde(rename = "counterpartyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterparty_id: Option<u64>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_add() {
        let json = r#"
        {
            "instrument": "BTC-USD",
            "clientOrderId": 123456789,
            "price": 999900000000,
            "size": 10000000,
            "side": "bid",
            "type": "limitIOC",
            "cod": false
        }
        "#;
        test_serde_value_type::<AddRequest>(json);

        let json = r#"
        {
            "id": 1245,
            "remainingSize": 8000000,
            "clientOrderId": 123456789,
            "cancelReason": 2,
            "deals": [
                {
                    "id": 23,
                    "price": 999900000000,
                    "size": 2000000,
                    "volume": 19998000000,
                    "delta": 0
                }
            ]
        }
        "#;
        test_serde_response::<AddResponse>(json);
        test_serde_response_err::<AddResponse>();
    }
}
