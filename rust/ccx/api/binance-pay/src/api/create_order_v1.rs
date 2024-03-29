use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::api::Api;
use crate::error::LibResult;
use crate::types::enums::TradeType;
use crate::types::time::Time;
use crate::uuid_simple;
use crate::BinancePayResponse;

const V1_BINANCEPAY_OPENAPI_ORDER: &str = "/binancepay/openapi/order";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1CreateOrderRequest {
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, // long	        Y	-   The merchant account id, issued when merchant been created at Binance.
    #[serde(rename = "subMerchantId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_merchant_id: Option<u64>, // long	        N	-   The sub merchant account id, issued when sub merchant been created at Binance.
    #[serde(rename = "merchantTradeNo", with = "uuid_simple")]
    pub merchant_trade_no: uuid::Uuid, // string	    Y	-   letter or digit, no other symbol allowed	The order id, Unique identifier for the request
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType, // string	    Y	-   "WEB", "APP"	operate entrance
    #[serde(rename = "totalFee")]
    pub total_fee: Decimal, // decimal	    Y	-   minimum unit: 0.01, minimum equivalent value: 0.5 USD	order amount
    pub currency: String, // string	    Y	-   only crypto token accepted, fiat NOT supported.	order currency, e.g. "BUSD"
    #[serde(rename = "productType")]
    pub product_type: String, // string	    Y	-   maximum length 16	product type
    #[serde(rename = "productName")]
    pub product_name: String, // string	    Y	-   maximum length 256	product name
    #[serde(rename = "productDetail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_detail: Option<String>, // string	    N	-   maximum length 256	product detail
    #[serde(rename = "returnUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>, // string	    N	-   maximum length 256	redirect url
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1OrderResult {
    #[serde(rename = "prepayId")]
    pub prepay_id: String, // string	    Y	-	unique id generated by binance
    #[serde(rename = "tradeType")]
    pub trade_type: String, // string	    Y	-   "WEB", "APP"	operate entrance
    #[serde(rename = "expireTime")]
    pub expire_time: i64, // string	    Y	-	expire time in milli seconds
    #[serde(rename = "qrcodeLink")]
    pub qrcode_link: String, // string	    Y	-	qr code img link
    #[serde(rename = "qrContent")]
    pub qr_content: String, // string	    Y	-	qr contend info
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v1_create_order(
        &self,
        request: V1CreateOrderRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<BinancePayResponse<V1OrderResult>> {
        self.client
            .post_json(V1_BINANCEPAY_OPENAPI_ORDER, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}
