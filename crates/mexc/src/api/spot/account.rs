use super::prelude::*;
use super::RlPriorityLevel;
use super::SymbolPermission;
use super::RL_ORDERS_PER_DAY;
use super::RL_ORDERS_PER_SECOND;
use super::RL_WEIGHT_PER_MINUTE;
use crate::client::Task;

pub const API_V3_ORDER_TEST: &str = "/api/v3/order/test";
pub const API_V3_ORDER: &str = "/api/v3/order";
pub const API_V3_OPEN_ORDERS: &str = "/api/v3/openOrders";
pub const API_V3_ALL_ORDERS: &str = "/api/v3/allOrders";
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
/// [Types of Orders](https://mexcdevelop.github.io/apidocs/spot_v3_en/#enum-definitions)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    LimitMaker,
    ImmediateOrCancel,
    FillOrKill,
}

// TODO: there is no docs on possible TimeInForce values
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

#[derive(Debug, Deserialize, Clone)]
pub struct NewTestOrder {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewOrder {
    pub symbol: Atom,
    pub order_id: String,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub price: Option<Decimal>,
    pub origQty: Option<Decimal>,
    pub r#type: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub transact_time: u64,
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order has been accepted by the engine.
    New,
    /// A part of the order has been filled.
    PartiallyFilled,
    /// The order has been completed.
    Filled,
    /// A part of the order has been cancelled
    PartiallyCanceled,
    /// The order has been canceled by the user.
    Canceled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelledOrder {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: String,
    // FIXME make None when -1.
    pub order_list_id: Option<i64>,
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
    pub order_id: String,
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
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub update_time: Option<u64>,
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
    pub id: String,
    pub order_id: String,
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
    pub is_self_trade: bool,
    pub client_order_id: Option<String>,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::RequestBuilder;

    impl<S> SpotApi<S>
    where
        S: crate::client::MexcSigner,
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
            quantity: Option<Decimal>,
            quote_order_qty: Option<Decimal>,
            price: Option<Decimal>,
            new_client_order_id: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<NewTestOrder>> {
            let request = self.prepare_order_request(
                symbol,
                side,
                r#type,
                quantity,
                quote_order_qty,
                price,
                new_client_order_id,
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
        /// Weight: 1
        ///
        #[allow(clippy::too_many_arguments)]
        pub fn create_order(
            &self,
            symbol: impl Serialize,
            side: OrderSide,
            r#type: OrderType,
            quantity: Option<Decimal>,
            quote_order_qty: Option<Decimal>,
            price: Option<Decimal>,
            new_client_order_id: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<NewOrder>> {
            let request = self.prepare_order_request(
                symbol,
                side,
                r#type,
                quantity,
                quote_order_qty,
                price,
                new_client_order_id,
                false,
                time_window,
            )?;

            Ok(self
                .rate_limiter
                .task(request)
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .cost(RL_ORDERS_PER_SECOND, 1)
                .cost(RL_ORDERS_PER_DAY, 1)
                .priority(RlPriorityLevel::High as u8)
                .send())
        }

        #[allow(clippy::too_many_arguments)]
        fn prepare_order_request(
            &self,
            symbol: impl Serialize,
            side: OrderSide,
            r#type: OrderType,
            quantity: Option<Decimal>,
            quote_order_qty: Option<Decimal>,
            price: Option<Decimal>,
            new_client_order_id: Option<impl Serialize>,
            is_test: bool,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<RequestBuilder<S>> {
            let endpoint = if is_test {
                API_V3_ORDER_TEST
            } else {
                API_V3_ORDER
            };
            match r#type {
                OrderType::Limit => {
                    if quantity.is_none() || price.is_none() {
                        Err(ApiError::mandatory_field_omitted("quantity, price"))?
                    }
                }
                OrderType::Market => {
                    if quantity.is_none() && quote_order_qty.is_none() {
                        Err(ApiError::mandatory_field_omitted(
                            "quantity or quote_order_qty",
                        ))?
                    }
                }
                _ => {}
            };
            let request = self
                .client
                .post(endpoint)?
                .signed(time_window)?
                .query_arg("symbol", &symbol)?
                .query_arg("side", &side)?
                .query_arg("type", &r#type)?
                .try_query_arg("quantity", &quantity)?
                .try_query_arg("quoteOrderQty", &quote_order_qty)?
                .try_query_arg("price", &price)?
                .try_query_arg("newClientOrderId", &new_client_order_id)?;

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
            order_id: Option<impl Serialize>,
            orig_client_order_id: Option<impl Serialize>,
            new_client_order_id: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<CancelledOrder>> {
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
        ) -> MexcResult<Task<Vec<CancelledOrder>>> {
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
            order_id: Option<impl Serialize>,
            orig_client_order_id: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Order>> {
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
        /// Weight(IP): 3;
        ///
        /// If the symbol is not sent, orders for all symbols will be returned in an array.
        pub fn open_orders(
            &self,
            symbol: impl AsRef<str>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<Order>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_OPEN_ORDERS)?
                        .signed(time_window)?
                        .query_arg("symbol", symbol.as_ref())?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 3)
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
        /// For some historical orders cummulativeQuoteQty will be < 0, meaning the data
        ///   is not available at this time.
        pub fn all_orders(
            &self,
            symbol: impl AsRef<str>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            limit: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<Order>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_ALL_ORDERS)?
                        .signed(time_window)?
                        .query_arg("symbol", symbol.as_ref())?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?
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
        ) -> MexcResult<Task<AccountInformation>> {
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
        /// * limit: Default 500; max 1000.
        ///
        /// If fromId is set, it will get id >= that fromId. Otherwise most recent trades are returned.
        pub fn my_trades(
            &self,
            symbol: impl AsRef<str>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            limit: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<MyTrade>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_MY_TRADES)?
                        .signed(time_window)?
                        .query_arg("symbol", symbol.as_ref())?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?
                        .try_query_arg("limit", &limit)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }
    }
}
