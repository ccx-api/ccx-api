use serde::Deserialize;
use serde::Serialize;

use crate::api::Api;
use crate::error::LibResult;
use crate::opt_uuid_simple;
use crate::types::enums::StatusRequest;
use crate::types::time::Time;

pub const BINANCEPAY_OPENAPI_ORDER_CLOSE: &str = "/binancepay/openapi/order/close";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2CloseOrderRequest {
    #[serde(rename = "merchantTradeNo", with = "opt_uuid_simple")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<uuid::Uuid>,
    #[serde(rename = "prepayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepay_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2CloseOrderResponse {
    pub status: StatusRequest, // string	            Y	"SUCCESS" or "FAIL"	status of the API request
    pub code: String,          // string	            Y	-	request result code, refer to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>, // bool         	    N	-	response body, refer to
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>, // string	            Y	-
}

impl V2CloseOrderResponse {
    pub fn is_success(&self) -> bool {
        self.status == StatusRequest::Success && self.data.unwrap_or(false)
    }
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v2_close_order(
        &self,
        request: V2CloseOrderRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<V2CloseOrderResponse> {
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
    fn test_serde_close_response() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": true,
            "errorMessage": null
        }
        "#;
        let response: V2CloseOrderResponse =
            serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_close_response response :: {:#?}", response);
    }
}
