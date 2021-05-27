use rust_decimal::Decimal;
use string_cache::DefaultAtom as Atom;

use super::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pong {}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: Atom,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: Atom,
    pub status: SymbolStatus,
    pub base_asset: Atom,
    pub base_asset_precision: u16,
    pub quote_asset: Atom,
    pub quote_precision: u16,
    pub quote_asset_precision: u16,
    pub base_commission_precision: u16,
    pub quote_commission_precision: u16,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filter>,
    pub permissions: Vec<SymbolPermission>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Bid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Ask {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggTrade {
    /// Aggregate tradeId.
    #[serde(rename = "a")]
    pub id: u64,
    /// Price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// Quantity.
    #[serde(rename = "q")]
    pub qty: Decimal,
    /// First tradeId.
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    /// Last tradeId.
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    /// Timestamp.
    #[serde(rename = "T")]
    pub time: u64,
    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    /// Was the trade the best price match?
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

// FIXME serialize as a tuple
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Kline {
    pub open_time: u64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub close_time: u64,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
    pub ignore: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AvgPrice {
    pub mins: u32,
    pub price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TickerStats {
    pub symbol: Atom,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub weighted_avg_price: Decimal,
    pub prev_close_price: Decimal,
    pub last_price: Decimal,
    pub last_qty: Decimal,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub volume: Decimal,
    pub quote_volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    /// First trade id.
    // FIXME Option<u64> when value is -1
    pub first_id: i64,
    /// Last trade id.
    // FIXME Option<u64> when value is -1
    pub last_id: i64,
    /// Trade count.
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct PriceTicker {
    pub symbol: Atom,
    pub price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: Atom,
    pub bid_price: Decimal,
    pub bid_qty: Decimal,
    pub ask_price: Decimal,
    pub ask_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ListenKey {
    pub listen_key: String,
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
    #[serde(rename = "type")]
    pub type_: OrderType,
    pub side: OrderSide,
    pub stop_price: Decimal,
    pub iceberg_qty: Decimal,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTestOrder {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NewOrder {
    Ack(NewOrderAck),
    Result(NewOrderResult),
}

impl NewOrder {
    pub fn as_ack(&self) -> Option<&NewOrderAck> {
        match self {
            NewOrder::Ack(order) => Some(order),
            _ => None
        }
    }

    pub fn as_result(&self) -> Option<&NewOrderResult> {
        match self {
            NewOrder::Result(order) => Some(order),
            _ => None
        }
    }
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
    #[serde(rename = "type")]
    pub type_: OrderType,
    pub side: OrderSide
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
    #[serde(rename = "type")]
    pub type_: OrderType,
    pub side: OrderSide,
}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct Transaction {
//    pub symbol: String,
//    pub order_id: u64,
//    pub client_order_id: String,
//    pub transact_time: u64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct UserDataStream {
//    pub listen_key: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct Success {}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//#[serde(untagged)]
//pub enum Prices {
//    AllPrices(Vec<SymbolPrice>),
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct SymbolPrice {
//    pub symbol: String,
//    #[serde(with = "string_or_float")] pub price: f64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//#[serde(untagged)]
//pub enum BookTickers {
//    AllBookTickers(Vec<Tickers>),
//}
//
//#[derive(Debug, Clone)]
//pub enum KlineSummaries {
//    AllKlineSummaries(Vec<KlineSummary>),
//}
//


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
    pub coin: Atom,
    pub tag: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewWithdraw {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    pub address: String,
    pub amount: Decimal,
    // FIXME decode time, example: "2021-04-29 16:08:00"
    pub apply_time: String,
    pub coin: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub transfer_type: Decimal,
    pub status: WithdrawStatus,
    pub tx_id: String,
}
