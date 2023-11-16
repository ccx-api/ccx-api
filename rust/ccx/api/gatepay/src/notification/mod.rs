use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

mod pay;
mod pay_actually;
mod pay_batch;
mod pay_refund;
mod received_delay_convert_address;
mod transfer_address;

pub use pay::*;
pub use pay_actually::*;
pub use pay_batch::*;
pub use pay_refund::*;
pub use received_delay_convert_address::*;
pub use transfer_address::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// Order ID
    pub biz_id: String,
    /// Order status, see the BizStatus table
    pub biz_status: BizStatus,
    /// Merchant client_id that created the order
    #[serde(alias = "client_id")]
    pub client_id: String,
    /// Message content, varies depending on the bizType
    #[serde(flatten)]
    pub data: BizType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "bizType", content = "data")]
pub enum BizType {
    /// Notification of non-address payment order status change to payment success PAY_SUCCESS,
    /// timeout, failure, or payment error, etc.
    Pay(Pay),
    /// Notification of refund order status change, refund success or failure.
    PayRefund(PayRefund),
    /// Notification of batch reward order status change.
    PayBatch(PayBatch),
    /// Notification of address payment order status change
    TransferAddress(TransferAddress),
    /// Notification of delayed payment order processing for address payments
    ReceivedConvertDelayAddress(ReceivedDelayConvertAddress),
    /// Notification of a payment order for revenue currency specified by the merchant.
    PayActually(PayActually),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BizStatus {
    /// Payment success
    #[serde(rename = "PAY_SUCCESS")]
    PaySuccess,
    /// Payment encountered an error
    #[serde(rename = "PAY_ERROR")]
    PayError,
    /// Order closed by merchant or timed out
    #[serde(rename = "PAY_CLOSE")]
    PayClose,
    /// Refund success
    #[serde(rename = "REFUND_SUCCESS")]
    RefundSuccess,
    /// Refund rejected
    #[serde(rename = "REFUND_REJECTED")]
    RefundRejected,
    /// Notification of an address payment order entering the PROCESS state
    #[serde(rename = "PAY_EXPIRED_IN_PROCESS")]
    PayExpiredInProcess,
    /// Address payment failed due to exchange rate fluctuations
    #[serde(rename = "PAY_EXPIRED_IN_EXCHANGE_FLUCTUATION")]
    PayExpiredInExchangeFluctuation,
    /// Successful transfer of address payment
    #[serde(rename = "TRANSFERRED_ADDRESS_PAID")]
    TransferredAddressPaid,
    /// Expired transfer of address payment
    #[serde(rename = "TRANSFERRED_ADDRESS_EXPIRE")]
    TransferredAddressExpire,
    /// Delayed transfer of address payment
    #[serde(rename = "TRANSFERRED_ADDRESS_DELAY")]
    TransferredAddressDelay,
    /// Delayed payment for a flash exchange, but no transfer was made.
    #[serde(rename = "CONVERT_ADDRESS_PAY_DELAY")]
    ConvertAddressPayDelay,
}
