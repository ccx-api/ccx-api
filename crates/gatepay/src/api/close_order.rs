use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderRequest {
    /// Merchant order number, up to 32 bytes
    pub merchant_trade_no: Option<SmartString<32>>,
    /// Order ID on the GatePay side
    pub prepay_id: Option<SmartString>,
}

impl Request for CloseOrderRequest {
    const METHOD: ApiMethod = ApiMethod::Post;
    const VERSION: ApiVersion = ApiVersion::V1;
    const PATH: &'static str = "pay/order/close";
    type Response = CloseOrderResponse;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderResponse {
    /// SUCCESS or FAIL
    result: CloseResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CloseResult {
    Success,
    Fail,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::GatepayApi;
    use crate::client::rest::RequestError;
    use crate::client::signer::GatepaySigner;

    impl<S: GatepaySigner> GatepayApi<S> {
        /// # Close pre-pay order
        ///
        /// Interface description: When the merchant's order is canceled, the merchant can initiate
        /// a request to GatePay to close an unpaid order. The unclosed order can only wait
        /// to go expire.
        pub async fn close_order(
            &self,
            merchant_trade_no: Option<SmartString<32>>,
            prepay_id: Option<SmartString>,
        ) -> Result<CloseOrderResponse, RequestError> {
            if !(merchant_trade_no.is_some() ^ prepay_id.is_some()) {
                return Err(RequestError::validate(
                    "Either merchant_trade_no or prepay_id must be set",
                ));
            }

            self.request(&CloseOrderRequest {
                merchant_trade_no,
                prepay_id,
            })
            .await
        }

        /// # Close pre-pay order
        ///
        /// Interface description: When the merchant's order is canceled, the merchant can initiate
        /// a request to GatePay to close an unpaid order. The unclosed order can only wait
        /// to go expire.
        #[inline]
        pub async fn close_order_by_prepay_id(
            &self,
            prepay_id: SmartString,
        ) -> Result<CloseOrderResponse, RequestError> {
            self.close_order(None, Some(prepay_id)).await
        }

        /// # Close pre-pay order
        ///
        /// Interface description: When the merchant's order is canceled, the merchant can initiate
        /// a request to GatePay to close an unpaid order. The unclosed order can only wait
        /// to go expire.
        #[inline]
        pub async fn close_order_by_merchant_trade_no(
            &self,
            merchant_trade_no: SmartString<32>,
        ) -> Result<CloseOrderResponse, RequestError> {
            self.close_order(Some(merchant_trade_no), None).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_example() {
        let json = r#"{
    "result": "SUCCESS"
}"#;
        let res: CloseOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            CloseOrderResponse {
                result: CloseResult::Success,
            }
        );
    }
}
