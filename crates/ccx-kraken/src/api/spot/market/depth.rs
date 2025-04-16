use std::collections::HashMap;

use bon::Builder;
use ccx_lib::order_book::{OrderBook, PriceAndAmount};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::prelude::CurrencyPair;
use crate::proto::{PublicRequest, Request, Response};

/// Get Depth Information.
///
/// Get depth information.
///
/// * pair - Asset pair to get data for.
/// * count - Maximum number of asks/bids. (optional, default: 100)
#[derive(Serialize, Debug, Builder)]
#[builder(on(CurrencyPair, into))]
pub struct Depth {
    pair: CurrencyPair,
    count: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetDepthInfo {
    pub asks: Vec<AssetDepthLotInfo>,
    pub bids: Vec<AssetDepthLotInfo>,
}

impl OrderBook for AssetDepthInfo {
    fn asks(&self) -> impl ExactSizeIterator<Item = ccx_lib::order_book::PriceAndAmount> {
        self.asks.iter().map(From::from)
    }

    fn bids(&self) -> impl ExactSizeIterator<Item = ccx_lib::order_book::PriceAndAmount> {
        self.bids.iter().map(From::from)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetDepthLotInfo {
    pub price: Decimal,
    pub volume: Decimal,
    pub timestamp: u32,
}

impl From<&AssetDepthLotInfo> for PriceAndAmount {
    fn from(value: &AssetDepthLotInfo) -> Self {
        PriceAndAmount {
            price: value.price,
            amount: value.volume,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DepthResponse {
    #[serde(flatten)]
    pub pairs: HashMap<CurrencyPair, AssetDepthInfo>,
}

impl Response for DepthResponse {}

impl Request for Depth {
    type Response = DepthResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/0/public/Depth";
}

impl PublicRequest for Depth {}
