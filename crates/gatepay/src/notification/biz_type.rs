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
pub enum BizType {
    /// Notification of non-address payment order status change to payment success PAY_SUCCESS,
    /// timeout, failure, or payment error, etc.
    Pay,
    /// Notification of refund order status change, refund success or failure.
    PayRefund,
    /// Notification of batch reward order status change.
    PayBatch,
    /// Notification of address payment order status change
    TransferAddress,
    /// Notification of delayed payment order processing for address payments
    ReceivedConvertDelayAddress,
    /// Notification of a payment order for revenue currency specified by the merchant.
    PayActually,
}

#[cfg(feature = "with_diesel_1-4")]
impl_diesel1!(BizType);
