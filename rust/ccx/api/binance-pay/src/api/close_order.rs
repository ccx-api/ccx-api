use serde::Deserialize;
use serde::Serialize;

use crate::api::Api;
use crate::error::LibResult;
use crate::opt_uuid_simple;
use crate::types::enums::StatusRequest;
use crate::types::time::Time;

pub const BINANCEPAY_OPENAPI_ORDER_CLOSE: &str = "/binancepay/openapi/order/close";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseOrderRequest {
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, //  long	Y	-	The merchant account id, issued when merchant been created at Binance.
    #[serde(rename = "subMerchantId")]
    pub sub_merchant_id: Option<u64>, //  long	N	-	The sub merchant account id, issued when sub merchant been created at Binance.
    #[serde(rename = "merchantTradeNo", with = "opt_uuid_simple")]
    pub merchant_trade_no: Option<uuid::Uuid>, //  string  N   -   letter or digit, no other symbol allowed, can not be empty if prepayId is empty	The order id, Unique identifier for the request
    #[serde(rename = "prepayId")]
    pub prepay_id: Option<String>, //  string	N	-   letter or digit, no other symbol allowed, can not be empty if merchantTradeNo is empty	Binance unique order id
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseOrderResponse {
    pub status: StatusRequest, // string	            Y	"SUCCESS" or "FAIL"	status of the API request
    pub code: String,          // string	            Y	-	request result code, refer to
    pub data: Option<bool>,    // bool         	    N	-	response body, refer to
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>, // string	            Y	-
}

impl CloseOrderResponse {
    pub fn is_success(&self) -> bool {
        self.status == StatusRequest::Success && self.data.unwrap_or(false)
    }
}

impl Api {
    pub async fn close_order(
        &self,
        request: CloseOrderRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<CloseOrderResponse> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_ORDER_CLOSE, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serde_close_request() {
        let json = r#"
        {
            "merchantId": 987321472,
            "subMerchantId": 987321472,
            "merchantTradeNo": "9a1c04a06dbc432e94fa4e2bd693c663",
            "prepayId": null
        }
        "#;
        let request: CloseOrderRequest = serde_json::from_str(json).expect("Failed from_str");
        println!("test_serde_close_request 1 :: {:#?}", request);

        let json = r#"
        {
            "merchantId": 987321472,
            "subMerchantId": 987321472,
            "merchantTradeNo": null,
            "prepayId": "9825382937292"
        }
        "#;
        let request: CloseOrderRequest = serde_json::from_str(json).expect("Failed from_str");
        println!("test_serde_close_request 2 :: {:#?}", request);

        let request = CloseOrderRequest {
            merchant_id: 134697918,
            sub_merchant_id: Some(134697918),
            merchant_trade_no: Some(
                uuid::Uuid::parse_str("9a1c04a0-6dbc-432e-94fa-4e2bd693c663")
                    .expect("Failed parse_str"),
            ),
            prepay_id: Some("99695089974435840".to_string()),
        };
        let json = serde_json::to_string(&request).expect("Failed to_string");
        println!("test_serde_close_request 3 :: {}", json);
    }

    #[test]
    fn test_serde_close_response() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": true,
            "errorMessage": null
        }
        "#;
        let response: CloseOrderResponse = serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_close_response response :: {:#?}", response);
    }
}
