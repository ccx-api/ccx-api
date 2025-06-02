use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

use super::{ClientOrderId, TxId, Userref};

/// Cancel a particular open order (or set of open orders).
///
/// Cancel a particular open order (or set of open orders) by txid, userref or cl_ord_id.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct CancelOrder {
    #[serde(flatten)]
    id: CancelOrderId,
}

#[derive(Debug, Clone, Serialize)]
pub enum CancelOrderId {
    /// Kraken order identifier
    #[serde(rename = "txid")]
    TxId(TxId),
    /// User reference id
    #[serde(rename = "txid")]
    Userref(Userref),
    /// An alphanumeric client order identifier which uniquely identifies an open order for each client.
    #[serde(rename = "cl_ord_id")]
    ClientOrderId(ClientOrderId),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CancelOrderResponse {
    /// Number of orders canceled
    pub count: i32,
    /// If set, order(s) is/are pending cancellation
    pub pending: Option<bool>,
}

impl Response for CancelOrderResponse {}

impl Request for CancelOrder {
    type Response = CancelOrderResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/CancelOrder";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for CancelOrder {}
