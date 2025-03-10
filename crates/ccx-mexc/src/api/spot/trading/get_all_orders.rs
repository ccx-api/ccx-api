use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::SignedRequest;

use crate::types::timestamp::MexcTimestamp;

use super::Order;

impl Request for GetAllOrders {
    type Response = Vec<Order>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/allOrders";
    const COST: u32 = 10;
}

impl SignedRequest for GetAllOrders {}

/// All Orders (USER_DATA)
///
/// Get all account orders; active, canceled, or filled.
///
/// Weight(IP): 10 with symbol
///
/// * limit: Default 500; max 1000.
///
/// For some historical orders cummulativeQuoteQty will be < 0, meaning the data
///   is not available at this time.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrders {
    symbol: SmartString,
    order_id: Option<SmartString>,
    start_time: Option<MexcTimestamp>,
    end_time: Option<MexcTimestamp>,
    limit: Option<u32>,
}

impl GetAllOrders {
    pub fn new(symbol: SmartString) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn new_with_time(
        symbol: SmartString,
        start_time: Option<MexcTimestamp>,
        end_time: Option<MexcTimestamp>,
    ) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time,
            end_time,
            limit: None,
        }
    }

    /// * limit — Default 500; max 1000.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}
