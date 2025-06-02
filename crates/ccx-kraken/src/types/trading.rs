use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::NoneAsEmptyString;
use serde_with::StringWithSeparator;
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::skip_serializing_none;

use super::currency_pair::CurrencyPair;

pub type Userref = i32;
pub type TxId = String;
pub type ClientOrderId = String;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(on(_, into))]
pub struct OrderParams {
    /// Client id to track kraken order (userref or cl_ord_id)
    #[serde(flatten)]
    client_id: Option<ClientId>,
    /// Order type
    ordertype: OrderType,
    /// Order direction (buy/sell)
    #[serde(rename = "type")]
    side: OrderSide,
    /// Order quantity in terms of the base asset
    #[serde_as(as = "DisplayFromStr")]
    volume: Decimal,
    /// For iceberg orders only, defines the quantity to show in the book
    #[serde_as(as = "NoneAsEmptyString")]
    displayvol: Option<Decimal>,
    /// Price (optional, for limit orders)
    #[serde_as(as = "NoneAsEmptyString")]
    price: Option<Decimal>,
    /// Secondary price (optional, for stop-loss, take-profit, stop-loss-limit, take-profit-limit orders)
    #[serde_as(as = "NoneAsEmptyString")]
    price2: Option<Decimal>,
    /// Price signal used to trigger stop orders (index/last, default: last)
    trigger: Option<TriggerType>,
    /// Amount of leverage desired (default: none)
    #[serde_as(as = "NoneAsEmptyString")]
    leverage: Option<Decimal>,
    /// If true, order will only reduce a currently open position
    reduce_only: Option<bool>,
    /// Self Trade Prevention mode
    stptype: Option<StpType>,
    /// Order flags (optional)
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, OrderFlag>>")]
    oflags: Option<Vec<OrderFlag>>,
    /// Time-in-force of the order
    timeinforce: Option<TimeInForce>,
    /// Scheduled start time (optional)
    starttm: Option<String>,
    /// Expiration time (optional)
    expiretm: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub enum ClientId {
    /// This is an optional non-unique, numeric identifier which can associated with a number of orders by the client. This field is mutually exclusive with cl_ord_id parameter.
    #[serde(rename = "userref")]
    Userref(Userref),
    /// Adds an alphanumeric client order identifier which uniquely identifies an open order for each client. This field is mutually exclusive with userref parameter.
    #[serde(rename = "cl_ord_id")]
    ClientOrderId(ClientOrderId),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Market,
    Limit,
    Iceberg,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    TrailingStop,
    TrailingStopLimit,
    SettlePosition,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderFlag {
    /// Post-only order (available when ordertype = limit)
    Post,
    /// Prefer fee in base currency (default if selling)
    Fcib,
    /// Prefer fee in quote currency (default if buying, mutually exclusive with fcib)
    Fciq,
    /// Disable market price protection for market orders
    Nompp,
    /// Order volume expressed in quote currency. This is supported only for market orders.
    Viqc,
}

serde_plain::derive_display_from_serialize!(OrderFlag);
serde_plain::derive_fromstr_from_deserialize!(OrderFlag);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TriggerType {
    Index,
    Last,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum StpType {
    /// Arriving order will be canceled
    CancelNewest,
    /// Resting order will be canceled
    CancelOldest,
    /// Both arriving and resting orders will be canceled
    CancelBoth,
}

#[derive(Serialize, Clone, Debug)]
pub enum TimeInForce {
    /// Good 'til canceled (default)
    #[serde(rename = "GTC")]
    GoodTilCanceled,
    /// Immediate or cancel
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    /// Good 'til date
    #[serde(rename = "GTD")]
    GoodTilDate,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct OrderDescription {
    /// Order description
    pub order: Option<String>,
    /// Conditional close order description, if applicable
    pub close: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct OrderInfo {
    /// Referral order transaction ID that created this order
    pub refid: Option<String>,
    /// User reference ID
    pub userref: Option<Userref>,
    /// Optional alphanumeric, client identifier associated with the order.
    pub cl_ord_id: Option<String>,
    /// Status of order
    pub status: OrderStatus,
    /// Unix timestamp of when order was placed
    pub opentm: u64,
    /// Unix timestamp of order start time (if set)
    pub starttm: Option<u64>,
    /// Unix timestamp of order end time (if set)
    pub expiretm: Option<u64>,
    /// Order description info
    pub descr: OrderInfoDescription,
    /// Volume of order (base currency unless viqc set in oflags)
    pub vol: Decimal,
    /// Volume executed (base currency unless viqc set in oflags)
    pub vol_exec: Decimal,
    /// Total cost (quote currency unless unless viqc set in oflags)
    pub cost: Decimal,
    /// Total fee (quote currency)
    pub fee: Decimal,
    /// Average price (quote currency unless viqc set in oflags)
    pub price: Decimal,
    /// Stop price (quote currency, for trailing stops)
    pub stopprice: Option<Decimal>,
    /// Triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<Decimal>,
    /// Price signal used to trigger stop orders (index/last, default: last)
    pub trigger: Option<TriggerType>,
    /// Indicates if the order is funded on margin.
    pub margin: Option<bool>,
    /// Additional info on status (if any)
    pub misc: String,
    /// For institutional accounts, identifies underlying sub-account/trader for Self Trade Prevention (STP).
    pub sender_sub_id: String,
    /// Comma delimited list of order flags
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, OrderFlag>>")]
    pub oflags: Option<Vec<OrderFlag>>,
    /// Order trades info (if "trades" input parameter is true)
    pub trades: Option<Vec<String>>,
    /// Unix timestamp of when order was closed
    pub closetm: Option<u64>,
    /// Additional info on status (if any)
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OrderStatus {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderInfoDescription {
    /// Asset pair
    pub pair: CurrencyPair,
    /// Type of order (buy/sell)
    #[serde(rename = "type")]
    pub side: OrderSide,
    /// Order type
    pub ordertype: OrderType,
    /// Primary price
    pub price: Decimal,
    /// Secondary price
    pub price2: Option<Decimal>,
    /// Amount of leverage
    pub leverage: Option<String>,
    /// Order description
    pub order: Option<String>,
    /// Conditional close order description (if conditional close set)
    pub close: Option<String>,
}
