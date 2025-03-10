use serde::Deserialize;
use serde::Serialize;

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

/// [Order List Status (listStatus Type)](https://developers.mexc.com/docs/mexc-spot-api-docs/enums#order-list-status-liststatustype)
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

/// [Order List Order Status (listOrderStatus)](https://developers.mexc.com/docs/mexc-spot-api-docs/enums#order-list-order-status-listorderstatus)
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

/// [ContingencyType](https://developers.mexc.com/docs/mexc-spot-api-docs/enums#contingencytype)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    /// OCO (One Cancels the Other)
    Oco,
    /// OTO (One Triggers the Other)
    Oto,
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

/// [Order side (side)](https://developers.mexc.com/docs/mexc-spot-api-docs/enums#order-side-side)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketSide {
    Buy,
    Sell,
}

// TODO: there is no docs on possible TimeInForce values
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

/// [STP Modes](https://developers.mexc.com/docs/mexc-spot-api-docs/enums#stp-modes)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    None,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
}
