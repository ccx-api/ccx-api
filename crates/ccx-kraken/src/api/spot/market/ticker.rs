use std::collections::HashMap;

use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::prelude::CurrencyPair;
use crate::proto::{PublicRequest, Request, Response};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::RateLimitType;

/// Get Ticker Information.
///
/// Note: Today's prices start at midnight UTC.
///
/// * pair - Asset pair to get data for.
#[derive(Serialize, Debug, Builder)]
#[builder(on(CurrencyPair, into))]
pub struct Ticker {
    pair: Option<CurrencyPair>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerLotInfo {
    pub price: Decimal,
    pub whole_lot_volume: Decimal,
    pub lot_volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerLastTradeInfo {
    pub price: Decimal,
    pub lot_volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerMetricInfo {
    pub today: Decimal,
    pub last_24_hours: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerTradesInfo {
    pub today: u32,
    pub last_24_hours: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerEntry {
    /// Ask.
    #[serde(rename = "a")]
    pub ask: TickerLotInfo,
    /// Bid.
    #[serde(rename = "b")]
    pub bid: TickerLotInfo,
    /// Last trade closed.
    #[serde(rename = "c")]
    pub close: TickerLastTradeInfo,
    /// Volume.
    #[serde(rename = "v")]
    pub volume: TickerMetricInfo,
    /// Volume weighted average price.
    #[serde(rename = "p")]
    pub volume_wa: TickerMetricInfo,
    /// Number of trades.
    #[serde(rename = "t")]
    pub trades: TickerTradesInfo,
    /// Low.
    #[serde(rename = "l")]
    pub low: TickerMetricInfo,
    /// High.
    #[serde(rename = "h")]
    pub high: TickerMetricInfo,
    /// Today's opening price.
    #[serde(rename = "o")]
    pub open: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerFee {
    pub volume: Decimal,
    pub percent_fee: Decimal,
}

#[derive(Deserialize, Debug)]
pub struct TickerResponse {
    #[serde(flatten)]
    pub ticker: HashMap<AssetName, TickerEntry>,
}

impl Response for TickerResponse {}

impl Request for Ticker {
    type Response = TickerResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/0/public/Ticker";

    const COSTS: &'static RateLimitType = &RateLimitType::Public;
}

impl PublicRequest for Ticker {}
