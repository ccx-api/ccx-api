use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

use super::{ClientOrderId, TxId};

/// Amend an existing order.
///
/// The amend request enables clients to modify the order parameters in-place
/// without the need to cancel the existing order and create a new one.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct AmendOrder {
    #[serde(flatten)]
    id: AmendOrderId,
    /// The new order quantity in terms of the base asset.
    order_qty: Option<String>,
    /// For iceberg orders only, defines the new quantity to show in the book while the rest remains hidden. Minimum value is 1/15 of remaining order quantity.
    display_qty: Option<String>,
    /// The new limit price restriction on the order (for order types that support limit price only). Supports relative pricing with +, - prefixes and/or % suffix.
    limit_price: Option<String>,
    /// The new trigger price to activate the order (for triggered order types only). Supports relative pricing with +, - prefixes and/or % suffix.
    trigger_price: Option<String>,
    /// An optional flag for limit_price amends. If true, the limit price change will be rejected if the order cannot be posted passively in the book.
    post_only: Option<bool>,
    /// RFC3339 timestamp after which the matching engine should reject the amend request, in presence of latency or order queueing. Min now() + 2 seconds, max now() + 60 seconds.
    deadline: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AmendOrderId {
    /// The Kraken identifier for the order to be amended. Either txid or cl_ord_id is required.
    #[serde(rename = "txid")]
    TxId(TxId),
    /// The client identifier for the order to be amended. Either txid or cl_ord_id is required.
    #[serde(rename = "cl_ord_id")]
    ClientOrderId(ClientOrderId),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AmendOrderResponse {
    /// Unique Kraken amend identifier
    pub amend_id: String,
}

impl Response for AmendOrderResponse {}

impl Request for AmendOrder {
    type Response = AmendOrderResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/AmendOrder";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for AmendOrder {}
