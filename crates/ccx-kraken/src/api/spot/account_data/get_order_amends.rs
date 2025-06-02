use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};
use crate::types::trading::TxId;

/// Get Order Amends.
///
/// Retrieves an audit trail of amend transactions on the specified order.
/// The list is ordered by ascending amend timestamp.
/// The first entry contains the original order parameters and has amend_type of "original".
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct GetOrderAmends {
    /// The Kraken order identifier.
    order_id: TxId,
}

/// Container for the audit trail of order amends.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GetOrderAmendsResponse {
    /// The total count of new and amend transactions (i.e. includes the original order).
    pub count: u32,
    /// Array of order amend entries, ordered by ascending amend timestamp.
    /// First entry contains original order parameters with amend_type "original".
    pub amends: Vec<OrderAmendEntry>,
}

/// Individual order amend entry in the audit trail.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct OrderAmendEntry {
    /// Kraken amend identifier
    pub amend_id: String,
    /// The type of amend transaction:
    /// • original: original order values on order entry.
    /// • user: user requested amendment.
    /// • restated: engine order maintenance amendment.
    pub amend_type: AmendType,
    /// Order quantity in terms of the base asset.
    pub order_qty: Option<Decimal>,
    /// The quantity show in the book for iceberg orders.
    pub display_qty: Option<Decimal>,
    /// Remaining un-traded quantity on the order.
    pub remaining_qty: Option<Decimal>,
    /// The limit price restriction on the order.
    pub limit_price: Option<Decimal>,
    /// The trigger price on trigger order types.
    pub trigger_price: Option<Decimal>,
    /// Description of the reason for this amend.
    pub reason: Option<String>,
    /// Indicates if the transaction was restricted from taking liquidity.
    pub post_only: Option<bool>,
    /// The UNIX timestamp for the amend transaction.
    pub timestamp: u64,
}

/// The type of amend transaction.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AmendType {
    /// Original order values on order entry.
    Original,
    /// User requested amendment.
    User,
    /// Engine order maintenance amendment.
    Restated,
}

impl Response for GetOrderAmendsResponse {}

impl Request for GetOrderAmends {
    type Response = GetOrderAmendsResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/OrderAmends";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::History);
}

impl SignedRequest for GetOrderAmends {}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn serialize_get_order_amends() {
            let get_order_amends = GetOrderAmends::builder()
                .order_id("OQCLML-BW3P3-BUCMWZ".to_string())
                .build();

            let actual = serde_json::to_value(&get_order_amends).unwrap();
            let expected = json!({
                "order_id": "OQCLML-BW3P3-BUCMWZ"
            });

            assert_eq!(actual, expected);
        }
    }
}
