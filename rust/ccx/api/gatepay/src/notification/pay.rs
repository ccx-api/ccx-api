use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::util::dt_gatepay::DtGatepay;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pay {
    /// Merchant trade number
    pub merchant_trade_no: String,
    /// goodsType when creating the order
    pub product_type: String,
    /// GoodsName when creating the order
    pub product_name: String,
    /// terminalType when creating the order
    pub trade_type: String,
    /// GoodsName when creating the order
    pub goods_name: String,
    /// terminalType when creating the order
    pub terminal_type: String,
    /// Order currency
    pub currency: String,
    /// Order amount
    pub total_fee: Decimal,
    /// Order amount
    pub order_amount: Decimal,
    /// Order creation time
    pub create_time: DtGatepay,
    /// Transaction ID
    pub transaction_id: String,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;
    use crate::notification::BizData;
    use crate::notification::BizStatus;
    use crate::notification::Notification;

    fn data_sample() -> Pay {
        Pay {
            merchant_trade_no: "gateio_withdraw6331782520222".to_string(),
            product_type: "NFT".to_string(),
            product_name: "ka".to_string(),
            trade_type: "APP".to_string(),
            goods_name: "ka".to_string(),
            terminal_type: "APP".to_string(),
            currency: "USDT".to_string(),
            total_fee: dec!(1.2),
            order_amount: dec!(1.2),
            create_time: DtGatepay::from_timestamp_ms(1664123708000),
            transaction_id: "24344545".to_string(),
        }
    }

    #[test]
    fn test_data() {
        let json = r#"{
            "merchantTradeNo":"gateio_withdraw6331782520222",
            "productType":"NFT",
            "productName":"ka",
            "tradeType":"APP",
            "goodsName":"ka",
            "terminalType":"APP",
            "currency":"USDT",
            "totalFee":"1.2",
            "orderAmount":"1.2",
            "createTime":1664123708000,
            "transactionId":"24344545"
        }"#;

        let data: Pay = serde_json::from_str(json).unwrap();

        assert_eq!(data, data_sample());
    }

    // #[test]
    fn _test_notification_example_from_docs() {
        let json = r#"{
            "bizType":"PAY",
            "bizId":"6948484859590",
            "bizStatus":"PAY_SUCCESS",
            "client_id":"cdhu-fgrfg44-5ggd-cdvsa",
            "data":{
                "merchantTradeNo":"gateio_withdraw6331782520222",
                "productType":"NFT",
                "productName":"ka",
                "tradeType":"APP",
                "goodsName":"ka",
                "terminalType":"APP",
                "currency":"USDT",
                "totalFee":"1.2",
                "orderAmount":"1.2",
                "createTime":1664123708000,
                "transactionId":"24344545"
            }
        }"#;

        let data: Notification = serde_json::from_str(json).unwrap();

        let sample = Notification {
            biz_id: "6948484859590".to_string(),
            biz_status: BizStatus::PaySuccess,
            client_id: "cdhu-fgrfg44-5ggd-cdvsa".to_string(),
            data: BizData::from(data_sample()),
        };

        assert_eq!(data, sample);
    }

    #[test]
    fn test_notification_real_notification() {
        let json = r#"{"bizType":"PAY","bizId":"167758445495193600","bizStatus":"PAY_SUCCESS","client_id":"1234567","data":"{\"merchantTradeNo\":\"928142df41ce47ce8786a22ca5fb6540\",\"productType\":\"\",\"productName\":\"928142df-41ce-47ce-8786-a22ca5fb6540\",\"tradeType\":\"WEB\",\"goodsName\":\"928142df-41ce-47ce-8786-a22ca5fb6540\",\"terminalType\":\"WEB\",\"currency\":\"USDT\",\"totalFee\":\"1.13\",\"orderAmount\":\"1.13\",\"payerId\":14392088,\"createTime\":1701958330207,\"transactionId\":\"167758445495193600\"}"}"#;

        let data: Notification = serde_json::from_str(json).unwrap();

        let sample = Notification {
            biz_id: "167758445495193600".to_string(),
            biz_status: BizStatus::PaySuccess,
            client_id: "1234567".to_string(),
            data: BizData::Pay(Pay {
                merchant_trade_no: "928142df41ce47ce8786a22ca5fb6540".to_string(),
                product_type: "".to_string(),
                product_name: "928142df-41ce-47ce-8786-a22ca5fb6540".to_string(),
                trade_type: "WEB".to_string(),
                goods_name: "928142df-41ce-47ce-8786-a22ca5fb6540".to_string(),
                terminal_type: "WEB".to_string(),
                currency: "USDT".to_string(),
                total_fee: dec!(1.13),
                order_amount: dec!(1.13),
                create_time: DtGatepay::from_timestamp_ms(1701958330207),
                transaction_id: "167758445495193600".to_string(),
            }),
        };

        assert_eq!(data, sample);
    }
}
