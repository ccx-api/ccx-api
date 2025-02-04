use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayRefund {
    /// Merchant trade number
    pub merchant_trade_no: String,
    /// Order amount
    pub order_amount: Decimal,
    /// Refund information
    pub refund_info: RefundInfo,
    /// Order currency
    pub currency: String,
    /// GoodsName when creating the order
    pub product_name: String,
    /// terminalType when creating the order
    pub terminal_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundInfo {
    /// Order amount
    pub order_amount: Decimal,
    /// Prepay ID
    pub prepay_id: String,
    /// Refund request ID
    pub refund_request_id: String,
    /// Refund amount
    pub refund_amount: Decimal,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;
    use crate::notification::BizData;
    use crate::notification::BizStatus;
    use crate::notification::Notification;

    fn data_sample() -> PayRefund {
        PayRefund {
            merchant_trade_no: "56236".to_string(),
            order_amount: dec!(1.91),
            refund_info: RefundInfo {
                order_amount: dec!(1.91),
                prepay_id: "1647438500687506".to_string(),
                refund_request_id: "156123911".to_string(),
                refund_amount: dec!(0.8),
            },
            currency: "BTC".to_string(),
            product_name: "NFT".to_string(),
            terminal_type: "MINIAPP".to_string(),
        }
    }

    #[test]
    fn test_data() {
        let json = r#"{
            "merchantTradeNo":"56236",
            "orderAmount":"1.91",
            "refundInfo":{
                "orderAmount":"1.91",
                "prepayId":"1647438500687506",
                "refundRequestId":"156123911",
                "refundAmount":"0.8"
            },
            "currency":"BTC",
            "productName":"NFT",
            "terminalType":"MINIAPP"
        }"#;

        let data: PayRefund = serde_json::from_str(json).unwrap();

        assert_eq!(data, data_sample());
    }

    // The json sample is from the official documentation:
    // https://www.gate.io/docs/gatepay/common/en/#_4-5-data-structure-corresponding-to-biztype
    // But it is broken, so this test is disabled.
    // #[test]
    fn _test_notification() {
        // TODO BizId is a string in the documentation and in other examples, but an integer
        //      in this one. Also, the client_id is missing.
        let json = r#"{
            "bizType":"PAY_REFUND",
            "bizId":123289163323899904,
            "bizStatus":"REFUND_SUCCESS",
            "data":{
                "merchantTradeNo":"56236",
                "orderAmount":"1.91",
                "refundInfo":{
                    "orderAmount":"1.91",
                    "prepayId":"1647438500687506",
                    "refundRequestId":"156123911",
                    "refundAmount":"0.8"
                },
                "currency":"BTC",
                "productName":"NFT",
                "terminalType":"MINIAPP"
            }
        }"#;

        let data: Notification = serde_json::from_str(json).unwrap();

        let sample = Notification {
            biz_id: "123289163323899904".to_string(),
            biz_status: BizStatus::RefundSuccess,
            // TODO: client_id is missing in the json sample.
            client_id: "".to_string(),
            data: BizData::from(data_sample()),
        };

        assert_eq!(data, sample);
    }
}
