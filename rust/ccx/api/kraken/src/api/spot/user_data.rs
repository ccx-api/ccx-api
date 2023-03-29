use std::collections::HashMap;

use super::prelude::*;
use crate::client::Task;

use super::{RL_MATCHING_ENGINE_PER_MINUTE, RL_PRIVATE_PER_MINUTE};

pub const API_0_PRIVATE_BALANCE: &str = "/0/private/Balance";
pub const API_0_PRIVATE_CLOSED_ORDERS: &str = "/0/private/ClosedOrders";
pub const API_0_PRIVATE_QUERY_ORDERS: &str = "/0/private/QueryOrders";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountBalanceResponse {
    /// Account Balance
    #[serde(flatten)]
    pub asset: HashMap<Atom, Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct GetClosedOrdersRequest {
    trades: Option<bool>,
    userref: Option<u32>,
    start: Option<f64>,
    end: Option<f64>,
    ofs: Option<u32>,
    closetime: Option<CloseTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GetClosedOrdersResponse {
    pub count: u32,
    pub closed: HashMap<String, OrderInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrderInfo {
    /// Referral order transaction ID that created this order.
    pub refid: Option<String>,
    /// User reference id.
    pub userref: Option<u32>,
    /// Status of order.
    pub status: OrderStatus,
    /// Additional info on status (if any).
    pub reason: Option<String>,
    /// Unix timestamp of when order was placed.
    pub opentm: f64,
    /// Unix timestamp of order start time (or 0 if not set).
    pub starttm: f64,
    /// Unix timestamp of order end time (or 0 if not set).
    pub expiretm: f64,
    /// Unix timestamp of when order was closed.
    pub closetm: Option<f64>,
    /// Order description info.
    pub descr: OrderDescription,
    /// Volume of order (base currency).
    pub vol: Decimal,
    /// Volume executed (base currency).
    pub vol_exec: Decimal,
    /// Total cost (quote currency unless).
    pub cost: Decimal,
    /// Total fee (quote currency).
    pub fee: Decimal,
    /// Average price (quote currency).
    pub price: Decimal,
    /// Stop price (quote currency).
    pub stopprice: Decimal,
    /// Triggered limit price (quote currency, when limit based order type triggered).
    pub limitprice: Decimal,
    /// Comma delimited list of miscellaneous info.
    pub misc: String,
    /// Comma delimited list of order flags.
    pub oflags: OrderFlags,
    /// List of trade IDs related to order (if trades info requested and data available).
    pub trades: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct OrderDescription {
    /// Asset pair.
    pub pair: Atom,
    /// Type of order (buy/sell).
    pub r#type: OrderSide,
    /// Order type.
    pub ordertype: OrderType,
    /// Primary price.
    pub price: Decimal,
    /// Secondary price.
    pub price2: Decimal,
    /// Amount of leverage.
    pub leverage: Option<String>,
    /// Order description.
    pub order: String,
    /// Conditional close order description (if conditional close set).
    pub close: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
struct QueryOrdersInfoRequest<'a> {
    trades: Option<bool>,
    userref: Option<u32>,
    txid: TxIds<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QueryOrdersInfoResponse {
    /// OpenOrder (object) or ClosedOrder (object).
    #[serde(flatten)]
    pub orders: HashMap<String, OrderInfo>,
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
        /// Get Account Balance.
        ///
        /// Retrieve all cash balances, net of pending withdrawals.
        pub fn get_account_balance(
            &self,
            nonce: Nonce,
        ) -> KrakenResult<Task<AccountBalanceResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_BALANCE)?
                        .signed(nonce)?
                        .request_body(())?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Get Closed Orders.
        ///
        /// Retrieve information about orders that have been closed (filled or cancelled). 50 results are returned at a time, the most recent by default.
        ///
        /// Note: If an order's tx ID is given for start or end time, the order's opening time (opentm) is used.
        #[allow(clippy::too_many_arguments)]
        pub fn get_closed_orders(
            &self,
            nonce: Nonce,
            trades: Option<bool>,
            userref: Option<u32>,
            start: Option<f64>,
            end: Option<f64>,
            ofs: Option<u32>,
            closetime: Option<CloseTime>,
        ) -> KrakenResult<Task<GetClosedOrdersResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_CLOSED_ORDERS)?
                        .signed(nonce)?
                        .request_body(GetClosedOrdersRequest {
                            trades,
                            userref,
                            start,
                            end,
                            ofs,
                            closetime,
                        })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .cost(RL_MATCHING_ENGINE_PER_MINUTE, 1)
                .send())
        }

        /// Query Orders Info .
        ///
        /// Retrieve information about specific orders..
        pub fn query_orders_info(
            &self,
            nonce: Nonce,
            trades: Option<bool>,
            userref: Option<u32>,
            txid: TxIds<'_>,
        ) -> KrakenResult<Task<QueryOrdersInfoResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_QUERY_ORDERS)?
                        .signed(nonce)?
                        .request_body(QueryOrdersInfoRequest {
                            trades,
                            userref,
                            txid,
                        })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .cost(RL_MATCHING_ENGINE_PER_MINUTE, 1)
                .send())
        }
    }
}
