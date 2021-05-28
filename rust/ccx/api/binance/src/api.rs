use rust_decimal::Decimal;
use serde::Serialize;

use crate::client::{ApiCred, Config, RestClient, WebsocketStream, RequestBuilder};
use crate::error::*;
use crate::proto::*;

const API_V3_PING: &str = "/api/v3/ping";
const API_V3_TIME: &str = "/api/v3/time";
const API_V3_EXCHANGE_INFO: &str = "/api/v3/exchangeInfo";
const API_V3_DEPTH: &str = "/api/v3/depth";
const API_V3_TRADES: &str = "/api/v3/trades";
const API_V3_HISTORICAL_TRADES: &str = "/api/v3/historicalTrades";
const API_V3_AGG_TRADES: &str = "/api/v3/aggTrades";
const API_V3_KLINES: &str = "/api/v3/klines";
const API_V3_AVG_PRICE: &str = "/api/v3/avgPrice";
const API_V3_TICKER_24HR: &str = "/api/v3/ticker/24hr";
const API_V3_TICKER_PRICE: &str = "/api/v3/ticker/price";
const API_V3_TICKER_BOOK_TICKER: &str = "/api/v3/ticker/bookTicker";
const V1_USER_DATA_STREAM: &str = "/api/v1/userDataStream";

const API_V3_ACCOUNT: &str = "/api/v3/account";
const API_V3_MY_TRADES: &str = "/api/v3/myTrades";
const API_V3_ALL_ORDERS: &str = "/api/v3/allOrders";
const API_V3_OPEN_ORDERS: &str = "/api/v3/openOrders";
const API_V3_ORDER: &str = "/api/v3/order";
const API_V3_ORDER_TEST: &str = "/api/v3/order/test";

const SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW: &str = "/sapi/v1/account/enableFastWithdrawSwitch";
const SAPI_V1_CAPITAL_DEPOSIT_ADDRESS: &str = "/sapi/v1/capital/deposit/address";
const SAPI_V1_CAPITAL_WITHDRAW_APPLY: &str = "/sapi/v1/capital/withdraw/apply";
const SAPI_V1_CAPITAL_WITHDRAW_HISTORY: &str = "/sapi/v1/capital/withdraw/history";

#[derive(Clone, Default)]
pub struct Api {
    pub client: RestClient,
}

impl Api {
    pub fn new() -> Self {
        Api::default()
    }

    pub fn from_env() -> Self {
        Api::with_config(Config::from_env())
    }

    pub fn with_cred(cred: ApiCred) -> Self {
        Api::with_config(Config {
            cred,
            ..Config::default()
        })
    }

    pub fn with_config(config: Config) -> Self {
        let client = RestClient::with_config(config);
        Api { client }
    }

    /// Test connectivity to the Rest API.
    ///
    /// Weight: 1
    pub async fn ping(&self) -> LibResult<Pong> {
        self.client.get(API_V3_PING)?.send().await
    }

    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// Weight: 1
    pub async fn time(&self) -> LibResult<ServerTime> {
        self.client.get(API_V3_TIME)?.send().await
    }

    /// Current exchange trading rules and symbol information.
    ///
    /// Weight: 1
    pub async fn exchange_info(&self) -> LibResult<ExchangeInformation> {
        self.client.get(API_V3_EXCHANGE_INFO)?.send().await
    }

    /// Order book.
    ///
    /// Weight: Adjusted based on the limit:
    ///
    /// Limit | Weight
    /// | ---- | ---- |
    /// 5, 10, 20, 50, 100 | 1
    /// 500 | 5
    /// 1000 | 10
    /// 5000 | 50
    ///
    /// The default `limit` value is `N100`.
    pub async fn depth<S: AsRef<str>>(
        &self,
        symbol: S,
        limit: impl Into<Option<OrderBookLimit>>,
    ) -> LibResult<OrderBook> {
        let limit = limit.into();
        self.client
            .get(API_V3_DEPTH)?
            .query_arg("symbol", symbol.as_ref())?
            .try_query_arg("limit", &limit.map(OrderBookLimit::as_str))?
            .send()
            .await
    }

    /// Recent trades list.
    ///
    /// Get recent trades.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    /// * `limit` - default 500; max 1000.
    ///
    /// Data Source: Memory
    pub async fn trades<S: AsRef<str>>(
        &self,
        symbol: S,
        limit: Option<usize>,
    ) -> LibResult<Vec<Trade>> {
        self.client
            .get(API_V3_TRADES)?
            .query_arg("symbol", symbol.as_ref())?
            .try_query_arg("limit", &limit)?
            .send()
            .await
    }

    /// Old Trade Lookup.
    ///
    /// Get older market trades.
    ///
    /// Weight: 5
    ///
    /// Parameters:
    /// * `symbol`
    /// * `from_id` - trade id to fetch from. Default gets most recent trades.
    /// * `limit` - default 500; max 1000.
    ///
    /// Data Source: Database
    pub async fn historical_trades<S: AsRef<str>>(
        &self,
        symbol: S,
        limit: Option<usize>,
        from_id: Option<u64>,
    ) -> LibResult<Vec<Trade>> {
        self.client
            .get(API_V3_HISTORICAL_TRADES)?
            .auth_header()?
            .query_arg("symbol", symbol.as_ref())?
            .try_query_arg("fromId", &from_id)?
            .try_query_arg("limit", &limit)?
            .send()
            .await
    }

    /// Compressed/Aggregate trades list.
    ///
    /// Get compressed, aggregate trades. Trades that fill at the time, from the same order,
    /// with the same price will have the quantity aggregated.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    /// * `from_id` - id to get aggregate trades from INCLUSIVE.
    /// * `start_time` - Timestamp in ms to get aggregate trades from INCLUSIVE.
    /// * `end_time` - timestamp in ms to get aggregate trades until INCLUSIVE.
    /// * `limit` - default 500; max 1000.
    ///
    ///
    /// * If both startTime and endTime are sent, time between startTime and endTime
    ///   must be less than 1 hour.
    /// * If fromId, startTime, and endTime are not sent, the most recent aggregate trades
    ///   will be returned.
    ///
    /// Data Source: Database
    pub async fn agg_trades<S: AsRef<str>>(
        &self,
        symbol: S,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<usize>,
    ) -> LibResult<Vec<AggTrade>> {
        self.client
            .get(API_V3_AGG_TRADES)?
            .query_arg("symbol", symbol.as_ref())?
            .try_query_arg("fromId", &from_id)?
            .try_query_arg("startTime", &start_time)?
            .try_query_arg("endTime", &end_time)?
            .try_query_arg("limit", &limit)?
            .send()
            .await
    }

    /// Kline/Candlestick data.
    ///
    /// Kline/candlestick bars for a symbol.
    /// Klines are uniquely identified by their open time.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    /// * `interval`
    /// * `start_time`
    /// * `end_time`
    /// * `limit` - default 500; max 1000.
    ///
    ///
    /// * If `start_time` and `end_time` are not sent, the most recent klines are returned.
    ///
    /// Data Source: Database
    pub async fn klines<S: AsRef<str>>(
        &self,
        symbol: S,
        interval: ChartInterval,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<usize>,
    ) -> LibResult<Vec<Kline>> {
        self.client
            .get(API_V3_KLINES)?
            .query_args(&[("symbol", symbol.as_ref()), ("interval", interval.as_str())])?
            .try_query_arg("startTime", &start_time)?
            .try_query_arg("endTime", &end_time)?
            .try_query_arg("limit", &limit)?
            .send()
            .await
    }

    /// Current average price.
    ///
    /// Current average price for a symbol.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    ///
    /// Data Source: Memory
    pub async fn avg_price<S: AsRef<str>>(&self, symbol: S) -> LibResult<AvgPrice> {
        self.client
            .get(API_V3_AVG_PRICE)?
            .query_arg("symbol", symbol.as_ref())?
            .send()
            .await
    }

    /// 24hr Ticker Price Change Statistics
    ///
    /// 24 hour rolling window price change statistics.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    ///
    /// Data Source: Memory
    pub async fn ticker_24hr<S: AsRef<str>>(&self, symbol: S) -> LibResult<TickerStats> {
        self.client
            .get(API_V3_TICKER_24HR)?
            .query_arg("symbol", symbol.as_ref())?
            .send()
            .await
    }

    /// 24hr Ticker Price Change Statistics
    ///
    /// 24 hour rolling window price change statistics.
    ///
    /// Weight: 40
    ///
    /// Data Source: Memory
    pub async fn ticker_24hr_all(&self) -> LibResult<Vec<TickerStats>> {
        self.client.get(API_V3_TICKER_24HR)?.send().await
    }

    /// Symbol price ticker.
    ///
    /// Latest price for a symbol.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    ///
    /// Data Source: Memory
    pub async fn ticker_price<S: AsRef<str>>(&self, symbol: S) -> LibResult<PriceTicker> {
        self.client
            .get(API_V3_TICKER_PRICE)?
            .query_arg("symbol", symbol.as_ref())?
            .send()
            .await
    }

    /// All symbol price tickers.
    ///
    /// Latest price for symbols.
    ///
    /// Weight: 2
    ///
    /// Data Source: Memory
    pub async fn ticker_price_all(&self) -> LibResult<Vec<PriceTicker>> {
        self.client.get(API_V3_TICKER_PRICE)?.send().await
    }

    /// Symbol order book ticker.
    ///
    /// Best price/qty on the order book for a symbol.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    ///
    /// Data Source: Memory
    pub async fn ticker_book<S: AsRef<str>>(&self, symbol: S) -> LibResult<BookTicker> {
        self.client
            .get(API_V3_TICKER_BOOK_TICKER)?
            .query_arg("symbol", symbol.as_ref())?
            .send()
            .await
    }

    /// All symbol order book tickers.
    ///
    /// Best price/qty on the order book for symbols.
    ///
    /// Weight: 2
    ///
    /// Data Source: Memory
    pub async fn ticker_book_all(&self) -> LibResult<Vec<BookTicker>> {
        self.client.get(API_V3_TICKER_BOOK_TICKER)?.send().await
    }

    /// Create a listenKey.
    ///
    /// Start a new user data stream.
    /// The stream will close after 60 minutes unless a keepalive is sent.
    ///
    /// Weight: 1
    pub async fn user_data_stream(&self) -> LibResult<ListenKey> {
        self.client
            .post(V1_USER_DATA_STREAM)?
            .auth_header()?
            .send()
            .await
    }

    /// Account Information (USER_DATA).
    ///
    /// Get current account information.
    ///
    /// Weight: 5
    pub async fn account(
        &self,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<AccountInformation> {
        self.client
            .get(API_V3_ACCOUNT)?
            .signed(time_window)?
            .send()
            .await
    }

    /// Account Trade List (USER_DATA).
    ///
    /// Get trades for a specific account and symbol.
    ///
    /// Weight: 5
    ///
    /// * from_id: TradeId to fetch from. Default gets most recent trades.
    /// * limit: Default 500; max 1000.
    ///
    /// If fromId is set, it will get id >= that fromId. Otherwise most recent trades are returned.
    pub async fn my_trades(
        &self,
        symbol: impl AsRef<str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        from_id: Option<u64>,
        limit: Option<u64>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<Vec<MyTrade>> {
        self.client
            .get(API_V3_MY_TRADES)?
            .signed(time_window)?
            .query_arg("symbol", symbol.as_ref())?
            .try_query_arg("startTime", &start_time)?
            .try_query_arg("endTime", &end_time)?
            .try_query_arg("fromId", &from_id)?
            .try_query_arg("limit", &limit)?
            .send()
            .await
    }

    /// All Orders (USER_DATA)
    ///
    /// Get all account orders; active, canceled, or filled.
    ///
    /// Weight: 5 with symbol
    ///
    /// * limit: Default 500; max 1000.
    ///
    /// If orderId is set, it will get orders >= that orderId. Otherwise most recent orders
    ///   are returned.
    /// For some historical orders cummulativeQuoteQty will be < 0, meaning the data
    ///   is not available at this time.
    /// If startTime and/or endTime provided, orderId is not required.
    pub async fn all_orders(
        &self,
        symbol: impl AsRef<str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        order_id: Option<u64>,
        limit: Option<u64>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<Vec<Order>> {
        self.client
            .get(API_V3_ALL_ORDERS)?
            .signed(time_window)?
            .query_arg("symbol", symbol.as_ref())?
            .try_query_arg("startTime", &start_time)?
            .try_query_arg("endTime", &end_time)?
            .try_query_arg("orderId", &order_id)?
            .try_query_arg("limit", &limit)?
            .send()
            .await
    }

    /// Current Open Orders (USER_DATA)
    ///
    /// Get all open orders on a symbol. Careful when accessing this with no symbol.
    ///
    /// Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
    ///
    /// If the symbol is not sent, orders for all symbols will be returned in an array.
    pub async fn open_orders(
        &self,
        symbol: Option<impl Serialize>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<Vec<Order>> {
        self.client
            .get(API_V3_OPEN_ORDERS)?
            .signed(time_window)?
            .try_query_arg("symbol", &symbol)?
            .send()
            .await
    }

    /// Test New Order (TRADE)
    ///
    /// Test new order creation and signature/recvWindow long.
    /// Creates and validates a new order but does not send it into the matching engine.
    ///
    /// Weight: 1
    ///
    /// Same as Api::order
    pub async fn create_order_test(
        &self,
        symbol: impl Serialize,
        side: OrderSide,
        type_: OrderType,
        time_in_force: Option<TimeInForce>,
        quantity: Option<Decimal>,
        quote_order_qty: Option<Decimal>,
        iceberg_qty: Option<Decimal>,
        price: Option<Decimal>,
        stop_price: Option<Decimal>,
        new_client_order_id: Option<impl Serialize>,
        new_order_resp_type: Option<OrderResponseType>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<NewTestOrder> {
        let request = self.prepare_order_request(
            symbol,
            side,
            type_,
            time_in_force,
            quantity,
            quote_order_qty,
            iceberg_qty,
            price,
            stop_price,
            new_client_order_id,
            new_order_resp_type,
            true,
            time_window
        )?;

        request.send().await
    }

    /// New Order (TRADE)
    ///
    /// Send in a new order.
    ///
    /// Weight: 1
    ///
    ///
    pub async fn create_order(
        &self,
        symbol: impl Serialize,
        side: OrderSide,
        type_: OrderType,
        time_in_force: Option<TimeInForce>,
        quantity: Option<Decimal>,
        quote_order_qty: Option<Decimal>,
        iceberg_qty: Option<Decimal>,
        price: Option<Decimal>,
        stop_price: Option<Decimal>,
        new_client_order_id: Option<impl Serialize>,
        new_order_resp_type: Option<OrderResponseType>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<NewOrder> {
        let request = self.prepare_order_request(
            symbol,
            side,
            type_,
            time_in_force,
            quantity,
            quote_order_qty,
            iceberg_qty,
            price,
            stop_price,
            new_client_order_id,
            new_order_resp_type,
            false,
            time_window
        )?;

        let new_order_resp_type = new_order_resp_type.unwrap_or_else(|| match type_ {
            OrderType::Limit | OrderType::Market => OrderResponseType::Full,
            _ => OrderResponseType::Ack,
        });

        Ok(match new_order_resp_type {
            OrderResponseType::Ack => NewOrder::Ack(request.send().await?),
            OrderResponseType::Result => NewOrder::Result(request.send().await?),
            OrderResponseType::Full => todo!(),
        })
    }

    fn prepare_order_request(
        &self,
        symbol: impl Serialize,
        side: OrderSide,
        type_: OrderType,
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
    ) -> LibResult<RequestBuilder> {
        let endpoint = if is_test {
            API_V3_ORDER_TEST
        } else {
            API_V3_ORDER
        };
        match type_ {
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
            .query_arg("type", &type_)?
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

    /// Query Order (USER_DATA)
    ///
    /// Check an order's status.
    ///
    /// Weight: 1
    ///
    /// Either orderId or origClientOrderId must be sent.
    /// For some historical orders cummulativeQuoteQty will be < 0,
    ///   meaning the data is not available at this time.
    pub async fn get_order(
        &self,
        symbol: impl Serialize,
        order_id: Option<u64>,
        orig_client_order_id: Option<impl Serialize>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<Order> {
        if order_id.is_none() && orig_client_order_id.is_none() {
            Err(ApiError::mandatory_field_omitted(
                "order_id or orig_client_order_id",
            ))?
        }
        self.client
            .get(API_V3_ORDER)?
            .signed(time_window)?
            .query_arg("symbol", &symbol)?
            .try_query_arg("orderId", &order_id)?
            .try_query_arg("origClientOrderId", &orig_client_order_id)?
            .send()
            .await
    }

    /// Cancel Order (TRADE)
    ///
    /// Cancel an active order.
    ///
    /// Weight: 1
    ///
    /// * newClientOrderId Used to uniquely identify this cancel. Automatically generated by default.
    ///
    /// Either orderId or origClientOrderId must be sent.
    pub async fn cancel_order(
        &self,
        symbol: impl Serialize,
        order_id: Option<u64>,
        orig_client_order_id: Option<impl Serialize>,
        new_client_order_id: Option<impl Serialize>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<CancelledOrder> {
        if order_id.is_none() && orig_client_order_id.is_none() {
            Err(ApiError::mandatory_field_omitted(
                "order_id or orig_client_order_id",
            ))?
        }
        self.client
            .delete(API_V3_ORDER)?
            .signed(time_window)?
            .query_arg("symbol", &symbol)?
            .try_query_arg("orderId", &order_id)?
            .try_query_arg("origClientOrderId", &orig_client_order_id)?
            .try_query_arg("newClientOrderId", &new_client_order_id)?
            .send()
            .await
    }

    /// Cancel all Open Orders on a Symbol (TRADE)
    ///
    /// Cancels all active orders on a symbol.
    /// This includes OCO orders.
    ///
    /// Weight: 1
    pub async fn cancel_all_orders(
        &self,
        symbol: impl Serialize,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<Vec<CancelledOrder>> {
        self.client
            .delete(API_V3_OPEN_ORDERS)?
            .signed(time_window)?
            .query_arg("symbol", &symbol)?
            .send()
            .await
    }

    /// Deposit Address (supporting network) (USER_DATA)
    ///
    /// Fetch deposit address with network.
    ///
    /// Weight: 1
    ///
    /// If network is not send, return with default network of the coin.
    /// You can get network and isDefault in networkList in the response of
    ///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
    pub async fn get_deposit_address(
        &self,
        coin: impl Serialize,
        network: Option<impl Serialize>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<DepositAddress> {
        self.client
            .get(SAPI_V1_CAPITAL_DEPOSIT_ADDRESS)?
            .signed(time_window)?
            .query_arg("coin", &coin)?
            .try_query_arg("network", &network)?
            .send()
            .await
    }

    /// Enable Fast Withdraw Switch (USER_DATA)
    ///
    /// Weight: 1
    ///
    /// This request will enable fastwithdraw switch under your account.
    /// You need to enable "trade" option for the api key which requests this endpoint.
    /// When Fast Withdraw Switch is on, transferring funds to a Binance account will be done
    ///   instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee.
    pub async fn enable_fast_withdraw_switch(
        &self,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<()> {
        self.client
            .post(SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW)?
            .signed(time_window)?
            .send_no_responce()
            .await
    }

    /// Withdraw(SAPI)
    ///
    /// Submit a withdraw request.
    ///
    /// Weight: 1
    ///
    /// * withdrawOrderId - client id for withdraw
    /// * addressTag - Secondary address identifier for coins like XRP,XMR etc.
    /// * transactionFeeFlag - When making internal transfer, true for returning the fee
    ///     to the destination account; false for returning the fee back to the departure account.
    ///     Default false.
    /// * name - Description of the address. Space in name should be encoded into %20.
    ///
    /// If network is not send, return with default network of the coin.
    /// You can get network and isDefault in networkList in the response of
    ///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
    pub async fn withdraw(
        &self,
        coin: impl Serialize,
        withdraw_order_id: Option<impl Serialize>,
        network: Option<impl Serialize>,
        address: impl Serialize,
        address_tag: Option<impl Serialize>,
        amount: Decimal,
        transaction_fee_flag: Option<bool>,
        name: Option<impl Serialize>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<NewWithdraw> {
        self.client
            .post(SAPI_V1_CAPITAL_WITHDRAW_APPLY)?
            .signed(time_window)?
            .query_arg("coin", &coin)?
            .try_query_arg("withdrawOrderId", &withdraw_order_id)?
            .try_query_arg("network", &network)?
            .query_arg("address", &address)?
            .try_query_arg("addressTag", &address_tag)?
            .query_arg("amount", &amount)?
            .try_query_arg("transactionFeeFlag", &transaction_fee_flag)?
            .try_query_arg("name", &name)?
            .send()
            .await
    }

    /// Withdraw History (supporting network) (USER_DATA)
    ///
    /// Fetch withdraw history.
    ///
    /// Weight: 1
    ///
    /// * startTime - Default: 90 days from current timestamp
    /// * endTime - Default: present timestamp
    ///
    /// * network may not be in the response for old withdraw.
    /// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
    /// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
    pub async fn withdraw_history(
        &self,
        coin: Option<impl Serialize>,
        status: Option<WithdrawStatus>,
        offset: Option<u16>,
        limit: Option<u16>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        time_window: impl Into<TimeWindow>,
    ) -> LibResult<Vec<Withdraw>> {
        self.client
            .get(SAPI_V1_CAPITAL_WITHDRAW_HISTORY)?
            .signed(time_window)?
            .try_query_arg("coin", &coin)?
            .try_query_arg("status", &status)?
            .try_query_arg("offset", &offset)?
            .try_query_arg("limit", &limit)?
            .try_query_arg("startTime", &start_time)?
            .try_query_arg("endTime", &end_time)?
            .send()
            .await
    }

    /// Creates multiplexed websocket stream.
    pub async fn ws(&self) -> LibResult<WebsocketStream> {
        self.client.web_socket2().await
    }
}
