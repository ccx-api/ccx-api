use std::collections::HashMap;

use super::prelude::*;
use super::RlPriorityLevel;
use crate::client::Task;
use crate::util::{Ask, Bid, OrderBook};

use super::RL_PUBLIC_PER_SECOND;

pub const API_0_PUBLIC_TIME: &str = "/0/public/Time";
pub const API_0_PUBLIC_SYSTEM_STATUS: &str = "/0/public/SystemStatus";
pub const API_0_PUBLIC_ASSETS: &str = "/0/public/Assets";
pub const API_0_PUBLIC_ASSET_PAIRS: &str = "/0/public/AssetPairs";
pub const API_0_PUBLIC_TICKER: &str = "/0/public/Ticker";
pub const API_0_PUBLIC_DEPTH: &str = "/0/public/Depth";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ServerTimeResponse {
    /// Unix timestamp
    pub unixtime: u64,
    // Sensless
    // /// RFC 1123 time format
    // pub rfc1123: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SystemStatusResponse {
    /// Current system status.
    ///
    /// * online - Kraken is operating normally. All order types may be submitted and trades can occur.
    /// * maintenance - The exchange is offline. No new orders or cancellations may be submitted.
    /// * cancel_only - Resting (open) orders can be cancelled but no new orders may be submitted. No trades will occur.
    /// * post_only - Only post-only limit orders can be submitted. Existing orders may still be cancelled. No trades will occur.
    pub status: SystemStatus,
    /// Current timestamp (RFC3339)
    // TODO pub timestamp: u64,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum SystemStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "cancel_only")]
    CancelOnly,
    #[serde(rename = "post_only")]
    PostOnly,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum AssetClass {
    #[serde(rename = "currency")]
    Currency,
    // TODO other classes ?
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetInfoResponse {
    #[serde(flatten)]
    pub asset: HashMap<Atom, AssetInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetInfo {
    /// Asset Class.
    pub aclass: AssetClass,
    /// Alternate name.
    pub altname: Atom,
    /// Scaling decimal places for record keeping.
    pub decimals: u32,
    /// Scaling decimal places for output display.
    pub display_decimals: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum AssetPairInfoKind {
    /// all info
    #[serde(rename = "info")]
    Info,
    /// leverage info
    #[serde(rename = "leverage")]
    Leverage,
    /// fees schedule
    #[serde(rename = "fees")]
    Fees,
    /// margin info
    #[serde(rename = "margin")]
    Margin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetPairResponse {
    #[serde(flatten)]
    pub pair: HashMap<Atom, AssetPairInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetPairInfo {
    /// Alternate pair name.
    pub altname: Atom,
    /// WebSocket pair name (if available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wsname: Option<Atom>,
    /// Asset class of base component.
    pub aclass_base: AssetClass,
    /// Asset ID of base component.
    pub base: Atom,
    /// Asset class of quote component.
    pub aclass_quote: AssetClass,
    /// Asset ID of quote component.
    pub quote: Atom,
    /// Volume lot size.
    #[deprecated]
    pub lot: Atom,
    /// Scaling decimal places for pair.
    pub pair_decimals: u32,
    /// Scaling decimal places for volume.
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
    pub fee_volume_currency: Atom,
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetTickerResponse {
    #[serde(flatten)]
    pub pair: HashMap<Atom, TickerInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TickerInfo {
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
pub struct AssetDepthResponse {
    #[serde(flatten)]
    pub pair: HashMap<Atom, AssetDepthInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetDepthInfo {
    pub asks: Vec<AssetDepthLotInfo>,
    pub bids: Vec<AssetDepthLotInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetDepthLotInfo {
    pub price: Decimal,
    pub volume: Decimal,
    pub timestamp: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderBookLimit {
    N5 = 5,
    N10 = 10,
    N20 = 20,
    N50 = 50,
    N100 = 100,
    N500 = 500,
}

impl OrderBookLimit {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

impl From<AssetDepthInfo> for OrderBook {
    fn from(value: AssetDepthInfo) -> Self {
        OrderBook {
            bids: value
                .bids
                .into_iter()
                .map(|i| Bid {
                    price: i.price,
                    qty: i.volume,
                    timestamp: i.timestamp.into(),
                    update_type: None,
                })
                .collect(),
            asks: value
                .asks
                .into_iter()
                .map(|i| Ask {
                    price: i.price,
                    qty: i.volume,
                    timestamp: i.timestamp.into(),
                    update_type: None,
                })
                .collect(),
        }
    }
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> SpotApi<S>
    where
        S: crate::client::KrakenSigner,
        S: Unpin + 'static,
    {
        /// Get Server Time.
        ///
        /// Get the server's time.
        pub fn time(&self) -> KrakenResult<Task<ServerTimeResponse>> {
            Ok(self
                .rate_limiter
                .task(self.client.get(API_0_PUBLIC_TIME)?)
                .cost(RL_PUBLIC_PER_SECOND, 1)
                .priority(RlPriorityLevel::Normal as u8)
                .send())
        }

        /// Get System Status.
        ///
        /// Get the current system status or trading mode.
        pub fn status(&self) -> KrakenResult<Task<SystemStatusResponse>> {
            Ok(self
                .rate_limiter
                .task(self.client.get(API_0_PUBLIC_SYSTEM_STATUS)?)
                .cost(RL_PUBLIC_PER_SECOND, 1)
                .priority(RlPriorityLevel::Normal as u8)
                .send())
        }

        /// Get Asset Info.
        ///
        /// Get information about the assets that are available for deposit, withdrawal, trading and staking.
        ///
        /// * assets - Comma delimited list of assets to get info on.
        /// * aclass - Asset class. (optional, default: currency)
        pub fn asset_info(
            &self,
            assets: Option<&str>,
            aclass: Option<AssetClass>,
        ) -> KrakenResult<Task<AssetInfoResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_0_PUBLIC_ASSETS)?
                        .try_query_arg("asset", &assets)?
                        .try_query_arg("aclass", &aclass)?,
                )
                .cost(RL_PUBLIC_PER_SECOND, 1)
                .send())
        }

        /// Get Tradable Asset Pairs.
        ///
        /// Get tradable asset pairs.
        ///
        /// * pairs - Asset pairs to get data for.
        pub fn asset_pairs(
            &self,
            pairs: Option<&str>,
            info: Option<AssetPairInfoKind>,
        ) -> KrakenResult<Task<AssetPairResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_0_PUBLIC_ASSET_PAIRS)?
                        .try_query_arg("pairs", &pairs)?
                        .try_query_arg("info", &info)?,
                )
                .cost(RL_PUBLIC_PER_SECOND, 1)
                .send())
        }

        /// Get Ticker Information.
        ///
        /// Note: Today's prices start at midnight UTC.
        ///
        /// * pair - Asset pair to get data for.
        pub fn ticker(&self, pair: &str) -> KrakenResult<Task<AssetTickerResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_0_PUBLIC_TICKER)?
                        .query_arg("pair", &pair)?,
                )
                .cost(RL_PUBLIC_PER_SECOND, 1)
                .send())
        }

        /// Get Depth Information.
        ///
        /// Get depth information.
        ///
        /// * pair - Asset pair to get data for.
        /// * count - Maximum number of asks/bids. (optional, default: 100)
        pub fn depth(
            &self,
            pair: &str,
            count: Option<OrderBookLimit>,
        ) -> KrakenResult<Task<AssetDepthResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_0_PUBLIC_DEPTH)?
                        .query_arg("pair", &pair)?
                        .try_query_arg("count", &count.map(|i| i.as_u16()))?,
                )
                .cost(RL_PUBLIC_PER_SECOND, 1)
                .send())
        }
    }
}
