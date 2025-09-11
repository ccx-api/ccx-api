use derive_more::From;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::Deserializer;
use serde::ser::SerializeStruct;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    /// Order ID
    pub biz_id: String,
    /// Order status, see the BizStatus table
    pub biz_status: BizStatus,
    /// Merchant client_id that created the order
    pub client_id: String,
    /// Message content, varies depending on the bizType
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

impl Serialize for Notification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Notification", 5)?;
        state.serialize_field("bizId", &self.biz_id)?;
        state.serialize_field("bizStatus", &self.biz_status)?;
        state.serialize_field("bizType", &self.data.as_biz_type())?;
        state.serialize_field("client_id", &self.client_id)?;
        let data = serde_json::to_string(&self.data).map_err(serde::ser::Error::custom)?;
        state.serialize_field("data", &data)?;
        state.end()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationInternal {
    /// Order ID
    pub biz_id: String,
    /// Order status, see the BizStatus table
    pub biz_status: BizStatus,
    /// Order type, see the BizType table
    pub biz_type: BizType,
    /// Merchant client_id that created the order
    #[serde(alias = "client_id")]
    pub client_id: String,
    /// Message content, varies depending on the bizType
    pub data: String,
}

impl<'de> Deserialize<'de> for Notification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let NotificationInternal {
            biz_id,
            biz_status,
            biz_type,
            client_id,
            data,
        } = NotificationInternal::deserialize(deserializer)?;

        let data = match biz_type {
            BizType::Pay => serde_json::from_str::<Pay>(&data)
                .map_err(de::Error::custom)?
                .into(),
            BizType::PayRefund => serde_json::from_str::<PayRefund>(&data)
                .map_err(de::Error::custom)?
                .into(),
            BizType::PayBatch => serde_json::from_str::<PayBatch>(&data)
                .map_err(de::Error::custom)?
                .into(),
            BizType::TransferAddress => serde_json::from_str::<TransferAddress>(&data)
                .map_err(de::Error::custom)?
                .into(),
            BizType::ReceivedConvertDelayAddress => {
                serde_json::from_str::<ReceivedDelayConvertAddress>(&data)
                    .map_err(de::Error::custom)?
                    .into()
            }
            BizType::PayActually => serde_json::from_str::<PayActually>(&data)
                .map_err(de::Error::custom)?
                .into(),
        };
        Ok(Notification {
            biz_id,
            biz_status,
            client_id,
            data,
        })
    }
}
