use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RateLimitType {
    #[serde(rename = "REQUEST_WEIGHT")]
    RequestWeight,
    #[serde(rename = "ORDERS")]
    Orders,
    #[serde(rename = "RAW_REQUESTS")]
    RawRequests,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RateLimitInterval {
    #[serde(rename = "SECOND")]
    Second,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "DAY")]
    Day,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChartInterval {
    #[serde(rename = "1m")]
    Minute1,
    #[serde(rename = "3m")]
    Minute3,
    #[serde(rename = "5m")]
    Minute5,
    #[serde(rename = "15m")]
    Minute15,
    #[serde(rename = "30m")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "8h")]
    Hour8,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "3d")]
    Day3,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl ChartInterval {
    pub fn as_str(self) -> &'static str {
        use ChartInterval::*;
        match self {
            Minute1 => "1m",
            Minute3 => "3m",
            Minute5 => "5m",
            Minute15 => "15m",
            Minute30 => "30m",
            Hour1 => "1h",
            Hour2 => "2h",
            Hour4 => "4h",
            Hour6 => "6h",
            Hour8 => "8h",
            Hour12 => "12h",
            Day1 => "1d",
            Day3 => "3d",
            Week1 => "1w",
            Month1 => "1M",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SymbolStatus {
    #[serde(rename = "PRE_TRADING")]
    PreTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "POST_TRADING")]
    PostTrading,
    #[serde(rename = "END_OF_DAY")]
    EndOfDay,
    #[serde(rename = "HALT")]
    Halt,
    #[serde(rename = "AUCTION_MATCH")]
    AuctionMatch,
    #[serde(rename = "BREAK")]
    Break,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SymbolType {
    #[serde(rename = "SPOT")]
    Spot,
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
pub enum OrderResponseType {
    #[serde(rename = "ACK")]
    Ack,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "FULL")]
    Full,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SymbolPermission {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "MARGIN")]
    Margin,
    #[serde(rename = "LEVERAGED")]
    Leveraged,
}

/// Filters define trading rules on a symbol or an exchange. Filters come in two forms:
/// symbol filters and exchange filters.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filter {
    /// The PRICE_FILTER defines the price rules for a symbol. There are 3 parts:
    ///
    /// * `min_price` defines the minimum `price`/`stop_price` allowed;
    ///   disabled on `min_price` == 0.
    /// * `max_price` defines the maximum `price`/`stop_price` allowed;
    ///   disabled on `max_price` == 0.
    /// * `tick_size` defines the intervals that a `price`/`stop_price`
    ///   can be increased/decreased by; disabled on `tick_size` == 0.
    ///
    /// Any of the above variables can be set to 0, which disables that rule in the price filter.
    /// In order to pass the price filter, the following must be true for `price`/`stop_price`
    /// of the enabled rules:
    ///
    /// * `price` >= `min_price`
    /// * `price` <= `max_price`
    /// * (`price` - `min_price`) % `tick_size` == 0
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: Decimal,
        max_price: Decimal,
        tick_size: Decimal,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        avg_price_mins: u64,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        min_notional: Decimal,
        apply_to_market: bool,
        avg_price_mins: u64,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u64 },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: u64 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u64 },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u64 },
}

// FIXME clarify: the documentation is ambiguous; only these values are listed as valid,
//       but below it has a caution about value 0.
//       [https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#order-book]
//       If 0 is valid, add it and specify its weight.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderBookLimit {
    N5 = 5,
    N10 = 10,
    N20 = 20,
    N50 = 50,
    N100 = 100,
    N500 = 500,
    N1000 = 1000,
    N5000 = 5000,
}

impl OrderBookLimit {
    pub fn weight(self) -> u32 {
        use OrderBookLimit as OBL;

        match self {
            OBL::N5 | OBL::N10 | OBL::N20 | OBL::N50 | OBL::N100 => 1,
            OBL::N500 => 5,
            OBL::N1000 => 10,
            OBL::N5000 => 50,
        }
    }

    pub fn as_str(self) -> &'static str {
        use OrderBookLimit as OBL;

        match self {
            OBL::N5 => "5",
            OBL::N10 => "10",
            OBL::N20 => "20",
            OBL::N50 => "50",
            OBL::N100 => "100",
            OBL::N500 => "500",
            OBL::N1000 => "1000",
            OBL::N5000 => "5000",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderBookStreamLimit {
    N5 = 5,
    N10 = 10,
    N20 = 20,
}

impl OrderBookStreamLimit {
    pub fn as_str(self) -> &'static str {
        use OrderBookStreamLimit::*;
        match self {
            N5 => "5",
            N10 => "10",
            N20 => "20",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AccountType {
    #[serde(rename = "SPOT")]
    Spot,
}

#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum WithdrawStatus {
    EmailSent = 0,
    Cancelled = 1,
    AwaitingApproval = 2,
    Rejected = 3,
    Processing = 4,
    Failure = 5,
    Completed = 6,
}

impl WithdrawStatus {
    pub fn is_finished(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(
            self,
            WS::Completed | WS::Cancelled | WS::Rejected | WS::Failure
        )
    }

    pub fn is_pending(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::EmailSent | WS::AwaitingApproval | WS::Processing)
    }

    pub fn needs_confirmation(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::EmailSent)
    }
}
