use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

mod biz_status;
mod biz_type;
mod pay;
mod pay_actually;
mod pay_batch;
mod pay_refund;
mod received_delay_convert_address;
mod transfer_address;

pub use biz_status::*;
pub use biz_type::*;
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
    pub data: BizData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "bizType", content = "data")]
pub enum BizData {
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

impl BizData {
    pub fn merchant_trade_no(&self) -> Option<&str> {
        match self {
            BizData::Pay(v) => Some(&v.merchant_trade_no),
            BizData::PayRefund(v) => Some(&v.merchant_trade_no),
            BizData::PayBatch(_v) => None,
            BizData::TransferAddress(v) => Some(&v.merchant_trade_no),
            BizData::ReceivedConvertDelayAddress(v) => Some(&v.merchant_trade_no),
            BizData::PayActually(v) => Some(&v.merchant_trade_no),
        }
    }

    pub fn as_biz_type(&self) -> BizType {
        match self {
            BizData::Pay(_) => BizType::Pay,
            BizData::PayRefund(_) => BizType::PayRefund,
            BizData::PayBatch(_) => BizType::PayBatch,
            BizData::TransferAddress(_) => BizType::TransferAddress,
            BizData::ReceivedConvertDelayAddress(_) => BizType::ReceivedConvertDelayAddress,
            BizData::PayActually(_) => BizType::PayActually,
        }
    }
}
