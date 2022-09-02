use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::api::Api;
use crate::error::LibResult;
use crate::opt_uuid_simple;
use crate::types::enums::StatusOrder;
use crate::types::enums::StatusRequest;
use crate::types::time::Time;
use crate::uuid_simple;

pub const V2_BINANCEPAY_OPENAPI_ORDER_QUERY: &str = "/binancepay/openapi/v2/order/query";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2QueryOrderRequest {
    #[serde(rename = "prepayId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepay_id: Option<String>,
    #[serde(rename = "merchantTradeNo", with = "opt_uuid_simple")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<uuid::Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2QueryOrderResult {
    #[serde(rename = "merchantId")]
    pub merchant_id: u64,
    #[serde(rename = "prepayId")]
    pub prepay_id: String,
    #[serde(rename = "transactionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "merchantTradeNo", with = "uuid_simple")]
    pub merchant_trade_no: uuid::Uuid,
    pub status: StatusOrder,
    pub currency: String,
    #[serde(rename = "orderAmount")]
    pub order_amount: Decimal,
    #[serde(rename = "openUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_user_id: Option<String>,
    #[serde(rename = "passThroughInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass_through_info: Option<String>,
    #[serde(rename = "transactTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<i64>,
    #[serde(rename = "createTime")]
    pub create_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2QueryOrderResponse {
    pub status: StatusRequest, // string	            Y	"SUCCESS" or "FAIL"	status of the API request
    pub code: String,          // string	            Y	-	request result code, refer to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<V2QueryOrderResult>, // QueryOrderResult	    N	-	response body, refer to
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>, // string	            Y	-
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v2_query_order(
        &self,
        request: V2QueryOrderRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<V2QueryOrderResponse> {
        self.client
            .post_json(V2_BINANCEPAY_OPENAPI_ORDER_QUERY, request)?
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
    fn test_serde_query_order_request_1() {
        let example = r#"
        {
            "merchantTradeNo": "2f4b0696-e691-43b5-94bb-ee4e4752d068"
        }
        "#;
        let response: V2QueryOrderRequest = serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_query_order_request_1 response :: {:#?}",
            response
        );
    }

    #[test]
    fn test_serde_query_order_request_2() {
        let example = r#"
        {
            "prepayId": "9825382937292"
        }
        "#;
        let response: V2QueryOrderRequest = serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_query_order_request_1 response :: {:#?}",
            response
        );
    }

    #[test]
    fn test_serde_query_order_response() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": {
              "merchantId": 98729382672,
              "prepayId": "383729303729303",
              "transactionId": "23729202729220282",
              "merchantTradeNo": "2f4b0696-e691-43b5-94bb-ee4e4752d068",
              "status": "PAID",
              "currency": "BUSD",
              "orderAmount": "10.88",
              "openUserId": "",
              "passThroughInfo": "",
              "transactTime": 1425744000123,
              "createTime": 1425744000000
            },
            "errorMessage": ""
        }
        "#;
        let response: V2QueryOrderResponse =
            serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_query_order_response response :: {:#?}",
            response
        );
    }
}
