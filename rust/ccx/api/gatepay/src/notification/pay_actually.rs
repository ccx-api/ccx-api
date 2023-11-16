use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::util::dt_gatepay::DtGatepay;
use crate::util::maybe_str;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayActually {
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
    /// Currency actually paid by the user
    pub pay_currency: String,
    /// Actual amount paid by the user
    pub pay_amount: Decimal,
    /// Currency specified for revenue by the merchant
    #[serde(with = "maybe_str")]
    pub expect_currency: Option<String>,
    /// Actual currency settled by Gate to the merchant. If Gate successfully converts
    /// the order currency to the currency requested by the merchant, actualCurrency
    /// will be the same as expectCurrency. Otherwise, actualCurrency will be equal to currency.
    #[serde(with = "maybe_str")]
    pub actual_currency: Option<String>,
    /// Amount corresponding to the actualCurrency currency
    #[serde(with = "maybe_str")]
    pub actual_amount: Option<Decimal>,
    /// UID of the paying user
    pub payer_id: u64,
    /// Order creation time
    pub create_time: DtGatepay,
    /// Transaction ID
    pub transaction_id: String,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;
    use crate::notification::BizStatus;
    use crate::notification::BizType;
    use crate::notification::Notification;

    fn data_sample() -> PayActually {
        PayActually {
            merchant_trade_no: "2345677666545556".to_string(),
            product_type: "NFT".to_string(),
            product_name: "NFT".to_string(),
            trade_type: "WEB".to_string(),
            goods_name: "NFT2".to_string(),
            terminal_type: "APP".to_string(),
            currency: "USDT".to_string(),
            total_fee: dec!(2.35),
            order_amount: dec!(2.35),
            pay_currency: "USDT".to_string(),
            pay_amount: dec!(2.36),
            expect_currency: None,
            actual_currency: None,
            actual_amount: None,
            payer_id: 10000,
            create_time: DtGatepay::from_timestamp(1676343810430),
            transaction_id: "59847585498494".to_string(),
        }
    }

    #[test]
    fn test_data() {
        let json = r#"{
            "merchantTradeNo":"2345677666545556",
            "productType":"NFT",
            "productName":"NFT",
            "tradeType":"WEB",
            "goodsName":"NFT2",
            "terminalType":"APP",
            "currency":"USDT",
            "totalFee":"2.35",
            "orderAmount":"2.35",
            "payCurrency":"USDT",
            "payAmount":"2.36",
            "expectCurrency":"",
            "actualCurrency":"",
            "actualAmount":"",
            "payerId":10000,
            "createTime":1676343810430,
            "transactionId":"59847585498494"
        }"#;

        let data: PayActually = serde_json::from_str(json).unwrap();

        assert_eq!(data, data_sample());
    }

    #[test]
    fn test_notification() {
        // TODO this sample is from the docs, but it is slightly corrected, because
        //   the docs contain a wrong value in the "bizType" field.
        let json = r#"{
            "bizType":"PAY_ACTUALLY",
            "bizId":"577886948403339870",
            "bizStatus":"PAY_SUCCESS",
            "client_id":"cdhu-fgrfg44-5ggd-cdvsa",
            "data":{
                "merchantTradeNo":"2345677666545556",
                "productType":"NFT",
                "productName":"NFT",
                "tradeType":"WEB",
                "goodsName":"NFT2",
                "terminalType":"APP",
                "currency":"USDT",
                "totalFee":"2.35",
                "orderAmount":"2.35",
                "payCurrency":"USDT",
                "payAmount":"2.36",
                "expectCurrency":"",
                "actualCurrency":"",
                "actualAmount":"",
                "payerId":10000,
                "createTime":1676343810430,
                "transactionId":"59847585498494"
            }
        }"#;

        let data: Notification = serde_json::from_str(json).unwrap();

        let sample = Notification {
            biz_id: "577886948403339870".to_string(),
            biz_status: BizStatus::PaySuccess,
            client_id: "cdhu-fgrfg44-5ggd-cdvsa".to_string(),
            data: BizType::from(data_sample()),
        };

        assert_eq!(data, sample);
    }
}
