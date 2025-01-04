use serde::Deserialize;
use serde::Serialize;

// Status	Description
// NEW	The order has been accepted by the engine.
// PENDING_NEW	The order is in a pending phase until the working order of an order list has been fully filled.
// PARTIALLY_FILLED	A part of the order has been filled.
// FILLED	The order has been completed.
// CANCELED	The order has been canceled by the user.
// PENDING_CANCEL	Currently unused
// REJECTED	The order was not accepted by the engine and not processed.
// EXPIRED	The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill)
// or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance)
// EXPIRED_IN_MATCH	The order was expired by the exchange due to STP. (e.g. an order with EXPIRE_TAKER will match with existing orders on the book with the same account or same tradeGroupId)

/// [Order Status (status)](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-status-status)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order has been accepted by the engine.
    New,
    /// The order is in a pending phase until the working order of an order list has been fully filled.
    PendingNew,
    /// A part of the order has been filled.
    PartiallyFilled,
    /// The order has been completed.
    Filled,
    /// The order has been canceled by the user.
    Canceled,
    /// Currently unused.
    PendingCancel,
    /// The order was not accepted by the engine and not processed.
    Rejected,
    /// The order was canceled according to the order type's rules
    /// (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill).
    Expired,
    /// The order was expired by the exchange due to STP
    /// (e.g. an order with EXPIRE_TAKER will match with existing orders on the book
    /// with the same account or same tradeGroupId).
    ExpiredInMatch,
}

/// [Order List Status (listStatus Type)](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-list-status-liststatustype)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderListStatus {
    /// This is used when the ListStatus is responding to a failed action.
    /// (E.g. order list placement or cancellation).
    Response,
    /// The order list has been placed or there is an update to the order list status.
    ExecStarted,
    /// The order list has finished executing and thus is no longer active.
    AllDone,
}

/// [Order List Order Status (listOrderStatus)](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-list-order-status-listorderstatus)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderListOrderStatus {
    /// Either an order list has been placed or there is an update to the status of the list.
    Executing,
    /// An order list has completed execution and thus no longer active.
    AllDone,
    /// The List Status is responding to a failed action either during order placement
    /// or order canceled.
    Reject,
}

/// [ContingencyType](https://developers.binance.com/docs/binance-spot-api-docs/enums#contingencytype)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    /// OCO (One Cancels the Other)
    Oco,
    /// OTO (One Triggers the Other)
    Oto,
}

/// [Order types (orderTypes, type)](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-types-ordertypes-type)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

/// [Order Response Type (newOrderRespType)](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-response-type-neworderresptype)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponseType {
    Ack,
    Result,
    Full,
}

/// [Order side (side)](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-side-side)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketSide {
    Buy,
    Sell,
}

/// [Time in force (timeInForce)](https://developers.binance.com/docs/binance-spot-api-docs/enums#time-in-force-timeinforce)
///
/// This sets how long an order will be active before expiration.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// Good Til Canceled.
    /// An order will be on the book unless the order is canceled.
    Gtc,
    /// Immediate Or Cancel.
    /// An order will try to fill the order as much as it can before the order expires.
    Ioc,
    /// Fill or Kill.
    /// An order will expire if the full order cannot be filled upon
    Fok,
}

/// [STP Modes](https://developers.binance.com/docs/binance-spot-api-docs/enums#stp-modes)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    None,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
}
