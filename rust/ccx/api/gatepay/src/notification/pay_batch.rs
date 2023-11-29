use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::util::dt_gatepay::DtGatepay;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayBatch {
    /// Merchant trade number
    pub merchant_batch_no: String,
    /// Order currency
    pub currency: String,
    /// Order list
    pub order_list: Vec<PayBatchOrder>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayBatchOrder {
    /// Receiver ID
    pub receiver_id: i64,
    /// Order amount
    pub amount: Decimal,
    /// Order currency
    pub currency: String,
    /// Order status
    pub status: String,
    /// Reward ID
    pub reward_id: String,
    /// Order creation time
    pub create_time: DtGatepay,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;
    use crate::notification::BizStatus;
    use crate::notification::BizType;
    use crate::notification::Notification;

    fn data_sample() -> PayBatch {
        PayBatch {
            merchant_batch_no: "6678554A99000".to_string(),
            currency: "".to_string(),
            order_list: vec![
                PayBatchOrder {
                    receiver_id: 10000,
                    amount: dec!(1.3),
                    currency: "USDT".to_string(),
                    status: "PAID".to_string(),
                    reward_id: "50888456789213330".to_string(),
                    create_time: DtGatepay::from_timestamp_ms(1676336326072),
                },
                PayBatchOrder {
                    receiver_id: 10001,
                    amount: dec!(5.7),
                    currency: "USDT".to_string(),
                    status: "PAID".to_string(),
                    reward_id: "50888456789215557".to_string(),
                    create_time: DtGatepay::from_timestamp_ms(1676336326072),
                },
            ],
        }
    }

    #[test]
    fn test_data() {
        let json = r#"{
            "merchant_batch_no":"6678554A99000",
            "currency":"",
            "order_list":[
                {
                    "receiver_id":10000,
                    "amount":"1.3",
                    "currency":"USDT",
                    "status":"PAID",
                    "reward_id":"50888456789213330",
                    "create_time":1676336326072
                },
                {
                    "receiver_id":10001,
                    "amount":"5.7",
                    "currency":"USDT",
                    "status":"PAID",
                    "reward_id":"50888456789215557",
                    "create_time":1676336326072
                }
            ]
        }"#;

        let data: PayBatch = serde_json::from_str(json).unwrap();

        assert_eq!(data, data_sample());
    }

    #[test]
    fn test_notification() {
        let json = r#"{
            "bizType":"PAY_BATCH",
            "bizId":"1234567999800",
            "bizStatus":"REFUND_SUCCESS",
            "client_id":"JaBxopuhY",
            "data":{
                "merchant_batch_no":"6678554A99000",
                "currency":"",
                "order_list":[
                    {
                        "receiver_id":10000,
                        "amount":"1.3",
                        "currency":"USDT",
                        "status":"PAID",
                        "reward_id":"50888456789213330",
                        "create_time":1676336326072
                    },
                    {
                        "receiver_id":10001,
                        "amount":"5.7",
                        "currency":"USDT",
                        "status":"PAID",
                        "reward_id":"50888456789215557",
                        "create_time":1676336326072
                    }
                ]
            }
        }"#;

        let data: Notification = serde_json::from_str(json).unwrap();

        let sample = Notification {
            biz_id: "1234567999800".to_string(),
            biz_status: BizStatus::RefundSuccess,
            client_id: "JaBxopuhY".to_string(),
            data: BizType::from(data_sample()),
        };

        assert_eq!(data, sample);
    }
}
