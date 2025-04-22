use std::collections::HashMap;

use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::StringWithSeparator;
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;

use crate::prelude::CurrencyPair;
use crate::proto::{PublicRequest, Request, Response};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::RateLimitType;

use super::AssetClass;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AssetPairInfo {
    /// all info
    Info,
    /// leverage info
    Leverage,
    /// fees schedule
    Fees,
    /// margin info
    Margin,
}

/// Get Tradable Asset Pairs.
///
/// Get tradable asset pairs.
///
/// * pairs - Asset pairs to get data for.
#[serde_as]
#[derive(Serialize, Debug, Builder)]
pub struct AssetPairs {
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pair: Option<Vec<String>>,
    info: Option<AssetPairInfo>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    country_code: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetPairEntry {
    /// Alternate pair name.
    pub altname: CurrencyPair,
    /// WebSocket pair name (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wsname: Option<CurrencyPair>,
    /// Asset class of base component.
    pub aclass_base: AssetClass,
    /// Asset ID of base component.
    pub base: AssetName,
    /// Asset class of quote component.
    pub aclass_quote: AssetClass,
    /// Asset ID of quote component.
    pub quote: AssetName,
    /// Number of decimal places for prices in this pair
    pub pair_decimals: u32,
    /// Number of decimal places for cost of trades in pair (quote asset terms)
    pub cost_decimals: u32,
    /// Number of decimal places for volume (base asset terms)
    pub lot_decimals: u32,
    /// Amount to multiply lot volume by to get currency volume.
    pub lot_multiplier: u32,
    /// Array of leverage amounts available when buying.
    pub leverage_buy: Vec<u32>,
    /// Array of leverage amounts available when selling.
    pub leverage_sell: Vec<u32>,
    /// Fee schedule.
    pub fees: Vec<AssetPairFee>,
    /// Maker fee schedule.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fees_maker: Vec<AssetPairFee>,
    /// Volume discount currency.
    pub fee_volume_currency: AssetName,
    /// Margin call level.
    pub margin_call: u32,
    /// Stop-out/liquidation margin level.
    pub margin_stop: u32,
    /// Minimum order size (in terms of base currency).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordermin: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetPairFee {
    pub volume: Decimal,
    pub percent_fee: Decimal,
}

#[derive(Deserialize, Debug)]
pub struct AssetPairsResponse {
    #[serde(flatten)]
    pub pairs: HashMap<CurrencyPair, AssetPairEntry>,
}

impl Response for AssetPairsResponse {}

impl Request for AssetPairs {
    type Response = AssetPairsResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/0/public/AssetPairs";

    const COSTS: &'static RateLimitType = &RateLimitType::Public;
}

impl PublicRequest for AssetPairs {}
