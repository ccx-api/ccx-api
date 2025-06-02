use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::StringWithSeparator;
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};
use crate::types::trading::{OrderInfo, TxId, Userref};

/// Query Orders Info.
///
/// Retrieve information about specific orders.
///
/// This endpoint can be used to get info on up to 50 orders.
/// For orders that were partially or fully filled and are no longer live (i.e., have a status of "closed" or "canceled"), this endpoint will return them with a status of "closed" irrespective of the actual fill.
/// For example, orders that are "canceled" with a partial fill will have status="closed" and vol_exec > 0.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct QueryOrderInfo {
    /// Whether or not to include trades related to position in output (default: false)
    trades: Option<bool>,
    /// Restrict results to given user reference id (optional)
    userref: Option<Userref>,
    /// The Kraken order identifier. To query multiple orders, use comma delimited list of up to 50 ids.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, TxId>")]
    txid: Vec<TxId>,
    /// Whether or not to consolidate trades by individual taker trades
    consolidate_taker: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QueryOrderInfoResponse {
    #[serde(flatten)]
    pub orders: HashMap<TxId, OrderInfo>,
}

impl Response for QueryOrderInfoResponse {}

impl Request for QueryOrderInfo {
    type Response = QueryOrderInfoResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/QueryOrders";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::History);
}

impl SignedRequest for QueryOrderInfo {}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn only_required_fields() {
            let query_order_info = QueryOrderInfo::builder()
                .txid(vec!["TXID123".to_string()])
                .build();

            let actual = serde_json::to_value(&query_order_info).unwrap();
            let expected = json!({
                "txid": "TXID123"
            });

            assert_eq!(actual, expected);
        }

        #[test]
        fn with_optional_fields() {
            let query_order_info = QueryOrderInfo::builder()
                .trades(true)
                .userref(12345)
                .txid(vec!["TXID123".to_string(), "TXID456".to_string()])
                .build();

            let actual = serde_json::to_value(&query_order_info).unwrap();
            let expected = json!({
                "trades": true,
                "userref": 12345,
                "txid": "TXID123,TXID456"
            });

            assert_eq!(actual, expected);
        }
    }
}
