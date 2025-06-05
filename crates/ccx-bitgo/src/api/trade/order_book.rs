use ccx_lib::order_book::PriceAndAmount;
use chrono::{DateTime, Utc};
use macro_rules_attribute::apply;

use crate::prelude::Product;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::derive::Response;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct OrderBook {
    /// The id of the trading account to retrieve
    #[serde(skip)]
    account_id: String,
    /// The name of the product (trading pair)
    #[serde(skip)]
    product: Product,
}

#[apply(Response)]
#[derive(PartialEq)]
pub struct OrderBookResponse {
    /// Timestamp of the order book snapshot
    pub time: DateTime<Utc>,
    /// Product name e.g. BTC-USD
    pub product: Product,
    /// An array of bid levels, each level containing [price, size]
    pub bids: Vec<PriceAndAmount>,
    /// An array of ask levels, each level containing [price, size]
    pub asks: Vec<PriceAndAmount>,
}

impl Response for OrderBookResponse {}

impl ccx_lib::order_book::OrderBook for OrderBookResponse {
    fn asks(&self) -> impl ExactSizeIterator<Item = PriceAndAmount> {
        self.asks.iter().copied()
    }

    fn bids(&self) -> impl ExactSizeIterator<Item = PriceAndAmount> {
        self.bids.iter().copied()
    }
}

impl Request for OrderBook {
    type Response = OrderBookResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;
        let product = &self.product;

        format!("/api/prime/trading/v1/accounts/{account_id}/products/{product}/level2").into()
    }
}

impl SignedRequest for OrderBook {}
