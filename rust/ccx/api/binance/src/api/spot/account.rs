use super::prelude::*;
use super::RlPriorityLevel;
use super::SymbolPermission;
use crate::client::Task;

use super::RL_ORDERS_PER_DAY;
use super::RL_ORDERS_PER_SECOND;
use super::RL_WEIGHT_PER_MINUTE;

pub const API_V3_ORDER_TEST: &str = "/api/v3/order/test";
pub const API_V3_ORDER: &str = "/api/v3/order";
// TODO pub const API_V3_ORDER_OCO: &str = "/api/v3/order/oco";
// TODO pub const API_V3_ORDER_LIST: &str = "/api/v3/orderList";
pub const API_V3_OPEN_ORDERS: &str = "/api/v3/openOrders";
pub const API_V3_ALL_ORDERS: &str = "/api/v3/allOrders";
// TODO pub const API_V3_ALL_ORDER_LIST: &str = "/api/v3/allOrderList";
// TODO pub const API_V3_OPEN_ORDER_LIST: &str = "/api/v3/openOrderList";
pub const API_V3_ACCOUNT: &str = "/api/v3/account";
pub const API_V3_MY_TRADES: &str = "/api/v3/myTrades";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

/// More information on how the order types definitions can be found here:
/// [Types of Orders](https://www.binance.com/en/support/articles/360033779452)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit = 1,
    #[serde(rename = "MARKET")]
    Market = 2,
    #[serde(rename = "STOP_LOSS")]
    StopLoss = 4,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit = 8,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit = 16,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit = 32,
    #[serde(rename = "LIMIT_MAKER")]
    LimitMaker = 64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TimeInForce {
    /// Good Til Canceled
    /// An order will be on the book unless the order is canceled.
    #[serde(rename = "GTC")]
    Gtc,
    /// Immediate Or Cancel
    /// An order will try to fill the order as much as it can before the order expires.
    #[serde(rename = "IOC")]
    Ioc,
    /// Fill or Kill
    /// An order will expire if the full order cannot be filled upon execution.
    #[serde(rename = "FOK")]
    Fok,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OrderResponseType {
    #[serde(rename = "ACK")]
    Ack,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "FULL")]
    Full,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewTestOrder {}

pub enum NewOrder {
    Ack(Task<NewOrderAck>),
    Result(Task<NewOrderResult>),
    Full(Task<NewOrderFull>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderAck {
    pub symbol: Atom,
    pub order_id: u64,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResult {
    pub symbol: Atom,
    pub order_id: u64,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderFull {
    pub symbol: Atom,
    pub order_id: u64,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub fills: Vec<OrderFill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderFill {
    pub price: Decimal,
    pub qty: Decimal,
    pub commission: Decimal,
    pub commission_asset: Atom,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OrderStatus {
    /// The order has been accepted by the engine.
    #[serde(rename = "NEW")]
    New,
    /// A part of the order has been filled.
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
    /// The order has been completed.
    #[serde(rename = "FILLED")]
    Filled,
    /// The order has been canceled by the user.
    #[serde(rename = "CANCELED")]
    Canceled,
    /// Currently unused.
    #[serde(rename = "PENDING_CANCEL")]
    PendingCancel,
    /// The order was not accepted by the engine and not processed.
    #[serde(rename = "REJECTED")]
    Rejected,
    /// The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with
    /// no fill, LIMIT IOC or MARKET orders that partially fill) or by the exchange, (e.g. orders
    /// canceled during liquidation, orders canceled during maintenance).
    #[serde(rename = "EXPIRED")]
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelledOrder {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u64,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    // FIXME make None when < 0.
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: Atom,
    pub order_id: u64,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    // FIXME make None when < 0.
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub stop_price: Decimal,
    pub iceberg_qty: Decimal,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: Decimal,
    pub taker_commission: Decimal,
    pub buyer_commission: Decimal,
    pub seller_commission: Decimal,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub update_time: u64,
    pub account_type: AccountType,
    pub balances: Vec<Balance>,
    // FIXME choose apropriate kind of permission.
    pub permissions: Vec<SymbolPermission>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AccountType {
    #[serde(rename = "SPOT")]
    Spot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: Atom,
    pub free: Decimal,
    pub locked: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MyTrade {
    pub symbol: Atom,
    pub id: u64,
    pub order_id: u64,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub commission: Decimal,
    pub commission_asset: Atom,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

impl NewOrder {
    pub fn is_ack(&self) -> bool {
        matches!(self, NewOrder::Ack(_))
    }

    pub fn is_result(&self) -> bool {
        matches!(self, NewOrder::Result(_))
    }

    pub fn is_full(&self) -> bool {
        matches!(self, NewOrder::Full(_))
    }

    pub fn as_ack(self) -> Option<Task<NewOrderAck>> {
        match self {
            NewOrder::Ack(order) => Some(order),
            _ => None,
        }
    }

    pub fn as_result(self) -> Option<Task<NewOrderResult>> {
        match self {
            NewOrder::Result(order) => Some(order),
            _ => None,
        }
    }

    pub fn as_full(self) -> Option<Task<NewOrderFull>> {
        match self {
            NewOrder::Full(order) => Some(order),
            _ => None,
        }
    }
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    use crate::client::RequestBuilder;

    impl<S> SpotApi<S>
    where
        S: crate::client::BinanceSigner,
        S: Unpin + 'static,
    {
        /// Test New Order (TRADE)
        ///
        /// Test new order creation and signature/recvWindow long.
        /// Creates and validates a new order but does not send it into the matching engine.
        ///
        /// Weight: 1
        ///
        /// Same as Api::order
        #[allow(clippy::too_many_arguments)]
        pub fn create_order_test(
            &self,
            symbol: impl Serialize,
            side: OrderSide,
            r#type: OrderType,
            time_in_force: Option<TimeInForce>,
            quantity: Option<Decimal>,
            quote_order_qty: Option<Decimal>,
            iceberg_qty: Option<Decimal>,
            price: Option<Decimal>,
            stop_price: Option<Decimal>,
            new_client_order_id: Option<impl Serialize>,
            new_order_resp_type: Option<OrderResponseType>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<NewTestOrder>> {
            let request = self.prepare_order_request(
                symbol,
                side,
                r#type,
                time_in_force,
                quantity,
                quote_order_qty,
                iceberg_qty,
                price,
                stop_price,
                new_client_order_id,
                new_order_resp_type,
                true,
                time_window,
            )?;

            Ok(self
                .rate_limiter
                .task(request)
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// New Order (TRADE)
        ///
        /// Send in a new order.
        ///
        /// Weight: 2
        ///
        ///
        #[allow(clippy::too_many_arguments)]
        pub fn create_order(
            &self,
            symbol: impl Serialize,
            side: OrderSide,
            r#type: OrderType,
            time_in_force: Option<TimeInForce>,
            quantity: Option<Decimal>,
            quote_order_qty: Option<Decimal>,
            iceberg_qty: Option<Decimal>,
            price: Option<Decimal>,
            stop_price: Option<Decimal>,
            new_client_order_id: Option<impl Serialize>,
            new_order_resp_type: Option<OrderResponseType>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<NewOrder> {
            let request = self.prepare_order_request(
                symbol,
                side,
                r#type,
                time_in_force,
                quantity,
                quote_order_qty,
                iceberg_qty,
                price,
                stop_price,
                new_client_order_id,
                new_order_resp_type,
                false,
                time_window,
            )?;

            let new_order_resp_type = new_order_resp_type.unwrap_or(match r#type {
                OrderType::Limit | OrderType::Market => OrderResponseType::Full,
                _ => OrderResponseType::Ack,
            });
            let task = self
                .rate_limiter
                .task(request)
                .cost(RL_WEIGHT_PER_MINUTE, 2)
                .cost(RL_ORDERS_PER_SECOND, 1)
                .cost(RL_ORDERS_PER_DAY, 1)
                .priority(RlPriorityLevel::High as u8);

            Ok(match new_order_resp_type {
                OrderResponseType::Ack => NewOrder::Ack(task.send::<NewOrderAck>()),
                OrderResponseType::Result => NewOrder::Result(task.send::<NewOrderResult>()),
                OrderResponseType::Full => NewOrder::Full(task.send::<NewOrderFull>()),
            })
        }

        #[allow(clippy::too_many_arguments)]
        fn prepare_order_request(
            &self,
            symbol: impl Serialize,
            side: OrderSide,
            r#type: OrderType,
            time_in_force: Option<TimeInForce>,
            quantity: Option<Decimal>,
            quote_order_qty: Option<Decimal>,
            iceberg_qty: Option<Decimal>,
            price: Option<Decimal>,
            stop_price: Option<Decimal>,
            new_client_order_id: Option<impl Serialize>,
            new_order_resp_type: Option<OrderResponseType>,
            is_test: bool,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<RequestBuilder<S>> {
            let endpoint = if is_test {
                API_V3_ORDER_TEST
            } else {
                API_V3_ORDER
            };
            match r#type {
                OrderType::Limit => {
                    if time_in_force.is_none() || quantity.is_none() || price.is_none() {
                        Err(ApiError::mandatory_field_omitted(
                            "time_in_force, quantity, price",
                        ))?
                    }
                }
                OrderType::Market => {
                    if quantity.is_none() && quote_order_qty.is_none() {
                        Err(ApiError::mandatory_field_omitted(
                            "quantity or quote_order_qty",
                        ))?
                    }
                }
                OrderType::StopLoss => {
                    if quantity.is_none() || stop_price.is_none() {
                        Err(ApiError::mandatory_field_omitted("quantity, stop_price"))?
                    }
                }
                OrderType::StopLossLimit => {
                    if time_in_force.is_none()
                        || quantity.is_none()
                        || price.is_none()
                        || stop_price.is_none()
                    {
                        Err(ApiError::mandatory_field_omitted(
                            "time_in_force, quantity, price, stop_price",
                        ))?
                    }
                }
                OrderType::TakeProfit => {
                    if quantity.is_none() || stop_price.is_none() {
                        Err(ApiError::mandatory_field_omitted("quantity, stop_price"))?
                    }
                }
                OrderType::TakeProfitLimit => {
                    if time_in_force.is_none()
                        || quantity.is_none()
                        || price.is_none()
                        || stop_price.is_none()
                    {
                        Err(ApiError::mandatory_field_omitted(
                            "time_in_force, quantity, price, stop_price",
                        ))?
                    }
                }
                OrderType::LimitMaker => {
                    if quantity.is_none() || price.is_none() {
                        Err(ApiError::mandatory_field_omitted("quantity, price"))?
                    }
                }
            };
            let request = self
                .client
                .post(endpoint)?
                .signed(time_window)?
                .query_arg("symbol", &symbol)?
                .query_arg("side", &side)?
                .query_arg("type", &r#type)?
                .try_query_arg("timeInForce", &time_in_force)?
                .try_query_arg("quantity", &quantity)?
                .try_query_arg("quoteOrderQty", &quote_order_qty)?
                .try_query_arg("icebergQty", &iceberg_qty)?
                .try_query_arg("price", &price)?
                .try_query_arg("stopPrice", &stop_price)?
                .try_query_arg("newClientOrderId", &new_client_order_id)?
                .try_query_arg("newOrderRespType", &new_order_resp_type)?;

            Ok(request)
        }

        /// Cancel Order (TRADE)
        ///
        /// Cancel an active order.
        ///
        /// Weight(IP): 1
        ///
        /// * newClientOrderId Used to uniquely identify this cancel. Automatically generated by default.
        ///
        /// Either orderId or origClientOrderId must be sent.
        pub fn cancel_order(
            &self,
            symbol: impl Serialize,
            order_id: Option<u64>,
            orig_client_order_id: Option<impl Serialize>,
            new_client_order_id: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<CancelledOrder>> {
            if order_id.is_none() && orig_client_order_id.is_none() {
                Err(ApiError::mandatory_field_omitted(
                    "order_id or orig_client_order_id",
                ))?
            }
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .delete(API_V3_ORDER)?
                        .signed(time_window)?
                        .query_arg("symbol", &symbol)?
                        .try_query_arg("orderId", &order_id)?
                        .try_query_arg("origClientOrderId", &orig_client_order_id)?
                        .try_query_arg("newClientOrderId", &new_client_order_id)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .priority(RlPriorityLevel::High as u8)
                .send())
        }

        /// Cancel all Open Orders on a Symbol (TRADE)
        ///
        /// Cancels all active orders on a symbol.
        /// This includes OCO orders.
        ///
        /// Weight(IP): 1
        pub fn cancel_all_orders(
            &self,
            symbol: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<CancelledOrder>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .delete(API_V3_OPEN_ORDERS)?
                        .signed(time_window)?
                        .query_arg("symbol", &symbol)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .priority(RlPriorityLevel::High as u8)
                .send())
        }

        /// Query Order (USER_DATA)
        ///
        /// Check an order's status.
        ///
        /// Weight(IP): 2
        ///
        /// Either orderId or origClientOrderId must be sent.
        /// For some historical orders cummulativeQuoteQty will be < 0,
        ///   meaning the data is not available at this time.
        pub fn get_order(
            &self,
            symbol: impl Serialize,
            order_id: Option<u64>,
            orig_client_order_id: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Order>> {
            if order_id.is_none() && orig_client_order_id.is_none() {
                Err(ApiError::mandatory_field_omitted(
                    "order_id or orig_client_order_id",
                ))?
            }
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_ORDER)?
                        .signed(time_window)?
                        .query_arg("symbol", &symbol)?
                        .try_query_arg("orderId", &order_id)?
                        .try_query_arg("origClientOrderId", &orig_client_order_id)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 2)
                .send())
        }

        /// Current Open Orders (USER_DATA)
        ///
        /// Get all open orders on a symbol. Careful when accessing this with no symbol.
        ///
        /// Weight(IP): 3 for a single symbol; 40 when the symbol parameter is omitted;
        ///
        /// If the symbol is not sent, orders for all symbols will be returned in an array.
        pub fn open_orders(
            &self,
            symbol: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<Order>>> {
            let weight: u32 = if symbol.is_some() { 3 } else { 40 };
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_OPEN_ORDERS)?
                        .signed(time_window)?
                        .try_query_arg("symbol", &symbol)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, weight)
                .send())
        }

        /// All Orders (USER_DATA)
        ///
        /// Get all account orders; active, canceled, or filled.
        ///
        /// Weight(IP): 10 with symbol
        ///
        /// * limit: Default 500; max 1000.
        ///
        /// If orderId is set, it will get orders >= that orderId. Otherwise most recent orders
        ///   are returned.
        /// For some historical orders cummulativeQuoteQty will be < 0, meaning the data
        ///   is not available at this time.
        /// If startTime and/or endTime provided, orderId is not required.
        pub fn all_orders(
            &self,
            symbol: impl AsRef<str>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            order_id: Option<u64>,
            limit: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<Order>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_ALL_ORDERS)?
                        .signed(time_window)?
                        .query_arg("symbol", symbol.as_ref())?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?
                        .try_query_arg("orderId", &order_id)?
                        .try_query_arg("limit", &limit)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        // TODO create_order_list
        // TODO cancel_order_list
        // TODO get_order_list
        // TODO all_order_list
        // TODO open_order_list

        /// Account Information (USER_DATA).
        ///
        /// Get current account information.
        ///
        /// Weight(IP): 10
        pub fn account(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AccountInformation>> {
            Ok(self
                .rate_limiter
                .task(self.client.get(API_V3_ACCOUNT)?.signed(time_window)?)
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        /// Account Trade List (USER_DATA).
        ///
        /// Get trades for a specific account and symbol.
        ///
        /// Weight(IP): 10
        ///
        /// * from_id: TradeId to fetch from. Default gets most recent trades.
        /// * limit: Default 500; max 1000.
        ///
        /// If fromId is set, it will get id >= that fromId. Otherwise most recent trades are returned.
        pub fn my_trades(
            &self,
            symbol: impl AsRef<str>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            from_id: Option<u64>,
            limit: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<MyTrade>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_MY_TRADES)?
                        .signed(time_window)?
                        .query_arg("symbol", symbol.as_ref())?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?
                        .try_query_arg("fromId", &from_id)?
                        .try_query_arg("limit", &limit)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }
    }
}
