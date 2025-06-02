use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

use super::{ClientOrderId, TxId};

/// Cancel multiple open orders in a single request.
///
/// Cancel multiple open orders (maximum 50) by txid, userref, or cl_ord_id.
/// This is more efficient than calling CancelOrder multiple times.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct CancelOrderBatch {
    /// List of order identifiers to cancel (maximum 50)
    #[serde(flatten)]
    orders: Option<Vec<CancelOrderIdBatch>>,
    cl_ord_ids: Option<Vec<ClientOrderIdBatch>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CancelOrderBatchResponse {
    /// Number of orders canceled
    pub count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub enum CancelOrderIdBatch {
    /// The Kraken identifier for the order to be amended. Either txid or cl_ord_id is required.
    #[serde(rename = "txid")]
    TxId(TxId),
    /// The client identifier for the order to be amended. Either txid or cl_ord_id is required.
    #[serde(rename = "cl_ord_id")]
    ClientOrderId(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientOrderIdBatch {
    pub cl_ord_id: ClientOrderId,
}

impl Response for CancelOrderBatchResponse {}

impl Request for CancelOrderBatch {
    type Response = CancelOrderBatchResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/CancelOrderBatch";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for CancelOrderBatch {}
