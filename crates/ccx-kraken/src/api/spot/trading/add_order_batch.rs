use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use crate::prelude::CurrencyPair;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;
use crate::types::trading::OrderParams;

use super::OrderDescription;

/// Add a batch of orders (minimum 2, maximum 15).
/// All orders in the batch must be for the same trading pair.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct AddOrderBatch {
    /// Asset pair id or altname (required, all orders must use the same pair)
    pair: CurrencyPair,
    /// List of orders to submit (minimum 2, maximum 15)
    orders: Vec<OrderParams>,
    /// RFC3339 timestamp after which the matching engine should reject the new order request (optional)
    deadline: Option<String>,
    /// Validate inputs only. Do not submit orders (optional)
    validate: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AddOrderBatchResponse {
    /// Array of order results in the same order as submitted
    pub orders: Vec<BatchOrderResult>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BatchOrderResult {
    /// Order description
    pub descr: Option<OrderDescription>,
    /// Array of transaction IDs for order
    #[serde(default)]
    pub txid: Vec<String>,
    /// Error message if order failed
    pub error: Option<String>,
}

impl Response for AddOrderBatchResponse {}

impl Request for AddOrderBatch {
    type Response = AddOrderBatchResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/AddOrderBatch";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for AddOrderBatch {}
