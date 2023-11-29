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
pub struct CreateOrderRequest {
    /// Merchant order number, up to 32 bytes
    pub merchant_trade_no: SmartString<32>,
    /// Order currency, uppercase, such as USDT, BTC, etc.
    pub currency: SmartString,
    /// Order amount, ranging from `[0.001, 500000]`
    pub order_amount: Decimal,
    /// The actual currency requested by the merchant for settlement. Use this field to specify
    /// the actual incoming currency if the settlement currency requested by the merchant is
    /// different from the order currency.
    pub actual_currency: Option<SmartString>,
    /// Transaction source APP, WEB, WAP, MINIAPP, OTHERS
    pub env: EnvType,
    /// Description of goods
    pub goods: GoodsType,
    /// Order expiration time, UTC timestamp in milliseconds. If not set, it defaults to 1 hour,
    /// with a maximum expiration time of 1 hour.
    pub order_expire_time: Option<DtGatepay>,
    /// The return URL for the order after successful payment, up to 256 characters long.
    pub return_url: Option<SmartString<126>>,
    /// The return URL for the order after payment failure, up to 256 characters long.
    pub cancel_url: Option<SmartString<126>>,
}

impl Request for CreateOrderRequest {
    const METHOD: ApiMethod = ApiMethod::Post;
    const VERSION: ApiVersion = ApiVersion::V1;
    const PATH: &'static str = "pay/transactions/native";
    type Response = CreateOrderResponse;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvType {
    pub terminal_type: TerminalType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TerminalType {
    App,
    Web,
    Wap,
    Miniapp,
    Others,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsType {
    /// Goods name, up to 160 characters.
    pub goods_name: SmartString<160>,
    /// Goods description, up to 256 characters.
    pub goods_detail: Option<SmartString<254>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    /// Order id on the platform.
    pub prepay_id: SmartString,
    /// Transaction source. Available values: APP, WEB, WAP, MINIAPP, OTHERS.
    pub terminal_type: TerminalType,
    /// Order expiration time in milliseconds. If not set, it defaults to 1 hour, with a maximum
    /// expiration time of 1 hour.
    pub expire_time: DtGatepay,
    /// The order QR code (valid for 1 hour) returned as a string. Developers need to use tools
    /// to generate the QR code image based on the content.
    pub qr_content: SmartString<254>,
    /// The Web payment component redirect URL after creating the order.
    pub location: SmartString<254>,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GatepaySigner;
    use crate::GatepayApi;

    impl<S: GatepaySigner> GatepayApi<S> {
        /// # Create a prepay order
        pub async fn create_order(
            &self,
            merchant_trade_no: SmartString<32>,
            currency: SmartString,
            order_amount: Decimal,
            actual_currency: Option<SmartString>,
            terminal_type: TerminalType,
            goods_name: SmartString<160>,
            goods_detail: Option<SmartString<254>>,
            order_expire_time: Option<DtGatepay>,
            return_url: Option<SmartString<126>>,
            cancel_url: Option<SmartString<126>>,
        ) -> Result<CreateOrderResponse, RequestError> {
            self.request(&CreateOrderRequest {
                merchant_trade_no,
                currency,
                order_amount,
                actual_currency,
                env: EnvType { terminal_type },
                goods: GoodsType {
                    goods_name,
                    goods_detail,
                },
                order_expire_time,
                return_url,
                cancel_url,
            })
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_example() {
        let json = r#"{
    "prepayId": "57477481708392448",
    "terminalType": "APP",
    "expireTime": 1675666173000,
    "qrContent": "http://openplatform.gate.io/qr/2Z29MDvtqIAeN5VuFLMK6IjEFBmv4V8bsKdDDWu2gLs=",
    "location": "https://114.55.238.130:13555/webpay?prepayid=57477481708392448"
}"#;
        let res: CreateOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            CreateOrderResponse {
                prepay_id: "57477481708392448".into(),
                terminal_type: TerminalType::App,
                expire_time: DtGatepay::from_timestamp_ms(1675666173000),
                qr_content:
                    "http://openplatform.gate.io/qr/2Z29MDvtqIAeN5VuFLMK6IjEFBmv4V8bsKdDDWu2gLs="
                        .into(),
                location: "https://114.55.238.130:13555/webpay?prepayid=57477481708392448".into(),
            }
        );
    }
}
