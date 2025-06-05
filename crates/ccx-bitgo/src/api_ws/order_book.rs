use macro_rules_attribute::apply;

use crate::prelude::Product;
use crate::types::derive::Request;

/// Order book WebSocket request payload
#[apply(Request)]
pub struct OrderBookRequest {
    account_id: String,
    product_id: Product,
}
