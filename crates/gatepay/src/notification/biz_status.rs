use serde::Deserialize;
use serde::Serialize;
use strum::AsRefStr;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "with_diesel_1-4",
    derive(diesel_derives::AsExpression, diesel_derives::FromSqlRow)
)]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
#[derive(EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum BizStatus {
    /// Payment success
    PaySuccess,
    /// Payment encountered an error
    PayError,
    /// Order closed by merchant or timed out
    PayClose,
    /// Refund success
    RefundSuccess,
    /// Refund rejected
    RefundRejected,
    /// Notification of an address payment order entering the PROCESS state
    PayExpiredInProcess,
    /// Address payment failed due to exchange rate fluctuations
    PayExpiredInExchangeFluctuation,
    /// Successful transfer of address payment
    TransferredAddressPaid,
    /// Expired transfer of address payment
    TransferredAddressExpire,
    /// Delayed transfer of address payment
    TransferredAddressDelay,
    /// Delayed payment for a flash exchange, but no transfer was made.
    ConvertAddressPayDelay,
}

#[cfg(feature = "with_diesel_1-4")]
impl_diesel1!(BizStatus);
