use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;
use crate::util::dt_gatepay::DtGatepay;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderStatusRequest {
    /// Merchant order number, up to 32 bytes
    pub merchant_trade_no: Option<SmartString<32>>,
    /// Order ID on the GatePay side
    pub prepay_id: Option<SmartString>,
}

impl Request for GetOrderStatusRequest {
    const METHOD: ApiMethod = ApiMethod::Post;
    const VERSION: ApiVersion = ApiVersion::V1;
    const PATH: &'static str = "pay/order/query";
    type Response = OrderStatusResponse;
}

// Attribute Name 	Type 	Required 	Description
// prepayId 	string 	Yes 	Prepay ID
// merchantId 	int64 	Yes 	Merchant user ID
// merchantTradeNo 	string 	Yes 	Merchant order ID
// transactionId 	string 	Yes 	Transaction ID
// goodsName 	string 	Yes 	Goods name
// currency 	string 	Yes 	Order currency
// orderAmount 	string 	Yes 	Order amount
// status 	string 	Yes 	Order status
// createTime 	int64 	Yes 	Prepay creation time in milliseconds
// expireTime 	int64 	Yes 	Prepay expiration time in milliseconds
// transactTime 	int64 	Yes 	Payment completion time in milliseconds
// order_name 	string 	Yes 	Order name
// pay_currency 	string 	Yes 	Currency paid by the user
// pay_amount 	string 	Yes 	Amount paid by the user
// expectCurrency 	string 	No 	Revenue currency specified when creating the order. Only returned in the order details with a specified settlement currency by the merchant.
// actualCurrency 	string 	No 	Currency actually settled to the merchant's account by the Gate backend after the order was paid. Only returned in the order details after Gate settles with the merchant.
// actualAmount 	string 	No 	Amount in the currency actually settled to the merchant's account by the Gate backend after the order was paid. Only returned in the order details after Gate settles with the merchant.
// rate 	string 	Yes 	Exchange rate for Flash Exchange payment.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusResponse {
    /// Prepay ID
    pub prepay_id: SmartString,
    /// Merchant user ID
    pub merchant_id: i64,
    /// Merchant order ID
    pub merchant_trade_no: SmartString<32>,
    /// Transaction ID
    pub transaction_id: SmartString,
    /// Goods name, up to 160 characters.
    pub goods_name: SmartString<160>,
    /// Goods description, up to 256 characters.
    pub goods_detail: Option<SmartString<254>>,
    /// Order currency, uppercase, such as USDT, BTC, etc.
    pub currency: SmartString,
    /// Order amount, ranging from `[0.001, 500000]`
    pub order_amount: Decimal,
    /// Order status
    pub status: OrderStatus,
    /// Prepay creation time in milliseconds
    pub create_time: DtGatepay,
    /// Prepay expiration time in milliseconds
    pub expire_time: DtGatepay,
    /// Payment completion time in milliseconds
    pub transact_time: DtGatepay,
    /// Order name
    #[serde(rename = "order_name")]
    pub order_name: SmartString,
    /// Currency paid by the user
    #[serde(rename = "pay_currency")]
    pub pay_currency: SmartString,
    /// Amount paid by the user
    #[serde(rename = "pay_amount")]
    pub pay_amount: Decimal,
    /// Revenue currency specified when creating the order. Only returned in the order details with a specified settlement currency by the merchant.
    pub expect_currency: Option<SmartString>,
    /// Currency actually settled to the merchant's account by the Gate backend after the order was paid. Only returned in the order details after Gate settles with the merchant.
    pub actual_currency: Option<SmartString>,
    /// Amount in the currency actually settled to the merchant's account by the Gate backend after the order was paid. Only returned in the order details after Gate settles with the merchant.
    pub actual_amount: Option<Decimal>,
    /// Exchange rate for Flash Exchange payment.
    pub rate: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    Paid,
    Expired,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GatepaySigner;
    use crate::GatepayApi;

    impl<S: GatepaySigner> GatepayApi<S> {
        /// # Order Status Inquiry
        ///
        /// In order to facilitate merchants to synchronize order information, GatePay provides
        /// a query interface for querying order status.
        pub async fn get_order_status(
            &self,
            merchant_trade_no: Option<SmartString<32>>,
            prepay_id: Option<SmartString>,
        ) -> Result<OrderStatusResponse, RequestError> {
            if !(merchant_trade_no.is_some() ^ prepay_id.is_some()) {
                return Err(RequestError::validate(
                    "Either merchant_trade_no or prepay_id must be set",
                ));
            }

            self.request(&GetOrderStatusRequest {
                merchant_trade_no,
                prepay_id,
            })
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_docs_example_of_response() {
        let json = r#"{
            "prepayId": "50620368071692288",
            "merchantId": 10002,
            "merchantTradeNo": "4379824792349592345",
            "transactionId": "",
            "goodsName": "NFT",
            "currency": "GT",
            "orderAmount": "0.1",
            "status": "EXPIRED",
            "createTime": 1674030436229,
            "expireTime": 1663054706000,
            "transactTime": 0,
            "order_name": "MiniApp-Payment#4379824792349592345",
            "pay_currency": "",
            "pay_amount": "0",
            "rate": "0"
        }"#;

        let res: OrderStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            OrderStatusResponse {
                prepay_id: "50620368071692288".into(),
                merchant_id: 10002,
                merchant_trade_no: "4379824792349592345".into(),
                transaction_id: "".into(),
                goods_name: "NFT".into(),
                goods_detail: None,
                currency: "GT".into(),
                order_amount: dec!(0.1),
                status: OrderStatus::Expired,
                create_time: DtGatepay::from_timestamp_ms(1674030436229),
                expire_time: DtGatepay::from_timestamp_ms(1663054706000),
                transact_time: DtGatepay::from_timestamp_ms(0),
                order_name: "MiniApp-Payment#4379824792349592345".into(),
                pay_currency: "".into(),
                pay_amount: dec!(0),
                expect_currency: None,
                actual_currency: None,
                actual_amount: None,
                rate: dec!(0),
            }
        );
    }

    /// Example response for querying the order details of a specified revenue currency that
    /// has not been settled:
    #[test]
    fn test_docs_example_not_settled() {
        let json = r#"{
            "prepayId": "56335302571069440",
            "merchantId": 10002,
            "merchantTradeNo": "118223456798",
            "transactionId": "",
            "goodsName": "NF2T",
            "currency": "USDT",
            "orderAmount": "1.9",
            "status": "EXPIRED",
            "createTime": 1675392982792,
            "expireTime": 1675396480000,
            "transactTime": 0,
            "order_name": "MiniApp-Payment#118223456798",
            "pay_currency": "",
            "pay_amount": "0",
            "expectCurrency": "BTC",
            "rate": "0"
        }"#;

        let res: OrderStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            OrderStatusResponse {
                prepay_id: "56335302571069440".into(),
                merchant_id: 10002,
                merchant_trade_no: "118223456798".into(),
                transaction_id: "".into(),
                goods_name: "NF2T".into(),
                goods_detail: None,
                currency: "USDT".into(),
                order_amount: dec!(1.9),
                status: OrderStatus::Expired,
                create_time: DtGatepay::from_timestamp_ms(1675392982792),
                expire_time: DtGatepay::from_timestamp_ms(1675396480000),
                transact_time: DtGatepay::from_timestamp_ms(0),
                order_name: "MiniApp-Payment#118223456798".into(),
                pay_currency: "".into(),
                pay_amount: dec!(0),
                expect_currency: Some("BTC".into()),
                actual_currency: None,
                actual_amount: None,
                rate: dec!(0),
            }
        );
    }

    /// Example response for querying the order details of a specified revenue currency that
    /// has been settled:
    #[test]
    fn test_docs_example_settled() {
        let json = r#"{
            "prepayId": "56416503889661952",
            "merchantId": 10002,
            "merchantTradeNo": "1182234567119",
            "transactionId": "56416503889661952",
            "goodsName": "NF2T",
            "currency": "GT",
            "orderAmount": "1",
            "status": "PAID",
            "createTime": 1675412342695,
            "expireTime": 1675414420000,
            "transactTime": 1675412385904,
            "order_name": "MiniApp-Payment#1182234567119",
            "pay_currency": "BTC",
            "pay_amount": "1",
            "expectCurrency": "USDT",
            "actualCurrency": "USDT",
            "actualAmount": "1",
            "rate": "1"
        }"#;

        let res: OrderStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            OrderStatusResponse {
                prepay_id: "56416503889661952".into(),
                merchant_id: 10002,
                merchant_trade_no: "1182234567119".into(),
                transaction_id: "56416503889661952".into(),
                goods_name: "NF2T".into(),
                goods_detail: None,
                currency: "GT".into(),
                order_amount: dec!(1),
                status: OrderStatus::Paid,
                create_time: DtGatepay::from_timestamp_ms(1675412342695),
                expire_time: DtGatepay::from_timestamp_ms(1675414420000),
                transact_time: DtGatepay::from_timestamp_ms(1675412385904),
                order_name: "MiniApp-Payment#1182234567119".into(),
                pay_currency: "BTC".into(),
                pay_amount: dec!(1),
                expect_currency: Some("USDT".into()),
                actual_currency: Some("USDT".into()),
                actual_amount: Some(dec!(1)),
                rate: dec!(1),
            }
        );
    }
}
