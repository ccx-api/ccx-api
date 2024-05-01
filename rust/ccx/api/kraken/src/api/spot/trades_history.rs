use std::collections::HashMap;
use std::marker::PhantomData;

use super::prelude::*;
use super::RlPriorityLevel;
use crate::client::Task;

use super::{RL_MATCHING_ENGINE_PER_MINUTE, RL_PRIVATE_PER_MINUTE};

pub const API_0_PRIVATE_TRADES_HISTORY: &str = "/0/private/TradesHistory";

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
struct TradesHistory {
    r#type: Option<TradesHistoryType>,
    trades: Option<bool>,
    start: Option<u64>,
    end: Option<u64>,
    ofs: Option<u64>,
    consolidate: Option<bool>,
    ledgers: Option<bool>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub enum TradesHistoryType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any position")]
    AnyPosition,
    #[serde(rename = "closed position")]
    ClosedPosition,
    #[serde(rename = "closing position")]
    ClosingPosition,
    #[serde(rename = "no position")]
    NoPosition,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradesHistoryResponse {
    /// Array of trade info.
    pub trades: HashMap<String, TradeInfo>,
    /// Current page number.
    pub count: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradeInfo {
    #[serde(rename = "ordertxid")]
    pub order_tx_id: String,
    #[serde(rename = "postxid")]
    pub pos_tx_id: String,
    /// Asset pair.
    pub pair: String,
    /// Unix timestamp of trade.
    pub time: f64,
    /// Type of order (buy/sell).
    pub r#type: TradeInfoType,
    /// Order type.
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    /// Average price order was executed at (quote currency).
    pub price: Decimal,
    /// Total cost of order (quote currency).
    pub cost: Decimal,
    /// Total fee (quote currency).
    pub fee: Decimal,
    /// Volume (base currency).
    pub vol: Decimal,
    /// Initial margin (quote currency).
    pub margin: Decimal,
    /// Amount of leverage used in trade.
    pub leverage: Option<String>,
    /// Comma delimited list of miscellaneous info.
    pub misc: String,
    /// List of ledger ids for entries associated with trade.
    pub ledgers: Option<Vec<String>>,
    /// Position trade id.
    pub trade_id: u64,
    /// Maker.
    pub maker: bool,
    /// Position status (open/closed)
    #[serde(rename = "posstatus")]
    pub pos_status: Option<String>,
    /// Average price of closed portion of position (quote currency)
    #[serde(rename = "cprice")]
    pub closed_price: Option<Decimal>,
    /// Total cost of closed portion of position (quote currency)
    #[serde(rename = "ccost")]
    pub closed_cost: Option<Decimal>,
    /// Total fee of closed portion of position (quote currency)
    #[serde(rename = "cfee")]
    pub closed_fee: Option<Decimal>,
    /// Total fee of closed portion of position (quote currency)
    #[serde(rename = "cvol")]
    pub closed_vol: Option<Decimal>,
    /// Total margin freed in closed portion of position (quote currency)
    #[serde(rename = "cmargin")]
    pub closed_margin: Option<Decimal>,
    /// Net profit/loss of closed portion of position (quote currency, quote currency scale)
    pub net: Option<Decimal>,
    /// List of closing trades for position (if available)
    #[serde(skip)]
    pub trades: Option<Vec<PhantomData<()>>>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TradeInfoType {
    Buy,
    Sell,
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
        /// Get Trades History
        ///
        /// Retrieve information about trades/fills.
        /// 50 results are returned at a time, the most recent by default.
        ///
        /// Unless otherwise stated, costs, fees, prices,
        /// and volumes are specified with the precision for the asset pair
        /// (pair_decimals and lot_decimals),
        /// not the individual assets' precision (decimals).
        #[allow(clippy::too_many_arguments)]
        pub fn trades_history(
            &self,
            nonce: Nonce,
            r#type: Option<TradesHistoryType>,
            trades: Option<bool>,
            start: Option<u64>,
            end: Option<u64>,
            ofs: Option<u64>,
            consolidate: Option<bool>,
            ledgers: Option<bool>,
        ) -> KrakenResult<Task<TradesHistoryResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_TRADES_HISTORY)?
                        .signed(nonce)?
                        .request_body(TradesHistory {
                            r#type,
                            trades,
                            start,
                            end,
                            ofs,
                            consolidate,
                            ledgers,
                        })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .priority(RlPriorityLevel::High as u8)
                .send())
        }
    }
}
