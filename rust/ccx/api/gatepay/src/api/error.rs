use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "UPPERCASE")]
pub enum GatepayResult<T> {
    Success(GatepayResponse<T>),
    Fail(GatepayApiError),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatepayResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatepayErrorInfo {
    pub label: String,
    #[serde(default)]
    pub error_message: String,
}

/// [source](https://www.gate.io/docs/gatepay/common/en/#_5-error-code)
///
/// | http status code | error code | description | solution |
/// | --- | --- | --- | --- |
/// | 500 | 300000 | System error | System exception, please retry with the same parameters |
/// | 500 | 300001 | Internal error | System exception, please retry with the same parameters |
/// | 500 | 400000 | Unknown error | System exception, please retry with the same parameters |
/// | 200 | 400001 | Request parameter format error | Check request data parameters and format |
/// | 200 | 400002 | Signature verification failed | Check if the merchant's signature is correct |
/// | 200 | 400003 | Request timestamp timed out | Check the timestamp field in the request header |
/// | 200 | 400004 | API identity key not found or invalid | _(missed in docs)_ Check whether the API identity key is correct |
/// | 200 | 400007 | Unsupported media type | Check the media type set in the interface |
/// | 200 | 400020 | Signature random number error | Please check whether the random number is empty |
/// | 200 | 400201 | Merchant order number already exists | Please verify whether the merchant order number has been submitted repeatedly |
/// | 200 | 400202 | Order does not exist | Check whether the order has been traded or whether the order number is correct |
/// | 200 | 400203 | Merchant number does not exist | Check whether the merchant number is correct |
/// | 200 | 400204 | Order status is incorrect | Check whether the order has expired, canceled, or closed, and use the query interface if necessary |
/// | 200 | 400205 | Invalid currency | Check the currency type of the order |
/// | 200 | 400304 | Refund ID does not exist | Check the requested refund ID |
/// | 200 | 400603 | Order timed out | Please verify whether the order has expired |
/// | 200 | 400604 | Invalid refund-related transaction order | Check whether the refund transaction order is in a completed state |
/// | 200 | 400605 | Insufficient balance in the payment account | Insufficient balance in the payment account |
/// | 200 | 400607 | Too many refunds | The number of refunds exceeds the limit |
/// | 200 | 400608 | Refund amount exception | Please check the refund amount |
/// | 200 | 400620 | Duplicate order payment | Please verify whether the merchant order number has been submitted repeatedly |
/// | 200 | 400621 | Incorrect payment amount | Check the requested amount |
/// | 200 | 400622 | Exchange rate fluctuations result in payment failure | You can try to apply again |
/// | 200 | 400623 | Unsupported currency payment | Check the payment currency |
/// | 200 | 400624 | Invalid order status notification address | Check whether the callback address provided by the merchant is valid |
/// | 200 | 500008 | Corresponding merchant not found | Check whether the requested merchant ID is correct |
/// | 200 | 500100 | Payment QR code expired | Redefine and generate a new QR code |
/// | 200 | 500101 | Duplicate payment QR code | Please verify the order status |
/// | 200 | 500103 | Address payment exchange currency error | Exchange rate fluctuations affect the collection rate |
/// | 200 | 500203 | Unable to query order details for address payment | Please check whether the address is correct |
/// | 200 | 500204 | Invalid recipient ID for refund transaction order | Please confirm that the recipient of the refund is a Gate user |
/// | 200 | 500205 | Refund currency does not match currency of the order or the user's payment currency | Please ensure that the refund currency is one of the order currency or the user's payment currency |
/// | 200 | 500206 | Refund amount exceeds limit | Please check the refund amount of the order |
/// | 200 | 500207 | Unable to find the refund order for address payment | Please confirm whether the refund was successful or check whether the address is correct |
/// | 200 | 500208 | Cannot refund orders without a converted address | Please confirm the type of refund order |

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Error)]
#[serde(tag = "code")]
pub enum GatepayApiError {
    /// System exception, please retry with the same parameters
    #[error("System exception, please retry with the same parameters {0:?}")]
    #[serde(rename = "300000")]
    SystemError(GatepayErrorInfo),

    /// System exception, please retry with the same parameters
    #[error("System exception, please retry with the same parameters {0:?}")]
    #[serde(rename = "300001")]
    InternalError(GatepayErrorInfo),

    /// System exception, please retry with the same parameters
    #[error("System exception, please retry with the same parameters {0:?}")]
    #[serde(rename = "400000")]
    UnknownError(GatepayErrorInfo),

    /// Check request data parameters and format
    #[error("Check request data parameters and format {0:?}")]
    #[serde(rename = "400001")]
    RequestParameterFormatError(GatepayErrorInfo),

    /// Check if the merchant's signature is correct
    #[error("Check if the merchant's signature is correct {0:?}")]
    #[serde(rename = "400002")]
    InvalidSignature(GatepayErrorInfo),

    /// Check the timestamp field in the request header
    #[error("Check the timestamp field in the request header {0:?}")]
    #[serde(rename = "400003")]
    RequestTimestampTimedOut(GatepayErrorInfo),

    /// Check whether the API identity key is correct
    #[error("Check whether the API identity key is correct {0:?}")]
    #[serde(rename = "400004")]
    ApiIdentityKeyNotFoundOrInvalid(GatepayErrorInfo),

    /// Check the media type set in the interface
    #[error("Check the media type set in the interface {0:?}")]
    #[serde(rename = "400007")]
    UnsupportedMediaType(GatepayErrorInfo),

    /// Please check whether the random number is empty
    #[error("Please check whether the random number is empty {0:?}")]
    #[serde(rename = "400020")]
    SignatureRandomNumberError(GatepayErrorInfo),

    /// Please verify whether the merchant order number has been submitted repeatedly
    #[error("Please verify whether the merchant order number has been submitted repeatedly {0:?}")]
    #[serde(rename = "400201")]
    MerchantOrderNumberAlreadyExists(GatepayErrorInfo),

    /// Check whether the order has been traded or whether the order number is correct
    #[error(
        "Check whether the order has been traded or whether the order number is correct {0:?}"
    )]
    #[serde(rename = "400202")]
    OrderDoesNotExist(GatepayErrorInfo),

    /// Check whether the merchant number is correct
    #[error("Check whether the merchant number is correct {0:?}")]
    #[serde(rename = "400203")]
    MerchantNumberDoesNotExist(GatepayErrorInfo),

    /// Check whether the order has expired, canceled, or closed, and use the query interface if necessary
    #[error("Check whether the order has expired, canceled, or closed, and use the query interface if necessary {0:?}")]
    #[serde(rename = "400204")]
    OrderStatusIsIncorrect(GatepayErrorInfo),

    /// Check the currency type of the order
    #[error("Check the currency type of the order {0:?}")]
    #[serde(rename = "400205")]
    InvalidCurrency(GatepayErrorInfo),

    /// Check the requested refund ID
    #[error("Check the requested refund ID {0:?}")]
    #[serde(rename = "400304")]
    RefundIdDoesNotExist(GatepayErrorInfo),

    /// Please verify whether the order has expired
    #[error("Please verify whether the order has expired {0:?}")]
    #[serde(rename = "400603")]
    OrderTimedOut(GatepayErrorInfo),

    /// Check whether the refund transaction order is in a completed state
    #[error("Check whether the refund transaction order is in a completed state {0:?}")]
    #[serde(rename = "400604")]
    InvalidRefundRelatedTransactionOrder(GatepayErrorInfo),

    /// Insufficient balance in the payment account
    #[error("Insufficient balance in the payment account {0:?}")]
    #[serde(rename = "400605")]
    InsufficientBalanceInThePaymentAccount(GatepayErrorInfo),

    /// The number of refunds exceeds the limit
    #[error("The number of refunds exceeds the limit {0:?}")]
    #[serde(rename = "400607")]
    TooManyRefunds(GatepayErrorInfo),

    /// Please check the refund amount
    #[error("Please check the refund amount {0:?}")]
    #[serde(rename = "400608")]
    RefundAmountException(GatepayErrorInfo),

    /// Please verify whether the merchant order number has been submitted repeatedly
    #[error("Please verify whether the merchant order number has been submitted repeatedly {0:?}")]
    #[serde(rename = "400620")]
    DuplicateOrderPayment(GatepayErrorInfo),

    /// Check the requested amount
    #[error("Check the requested amount {0:?}")]
    #[serde(rename = "400621")]
    IncorrectPaymentAmount(GatepayErrorInfo),

    /// Exchange rate fluctuations affect the collection rate. You can try to apply again
    #[error("Exchange rate fluctuations affect the collection rate {0:?}")]
    #[serde(rename = "400622")]
    ExchangeRateFluctuationsResultInPaymentFailure(GatepayErrorInfo),

    /// Check the payment currency
    #[error("Check the payment currency {0:?}")]
    #[serde(rename = "400623")]
    UnsupportedCurrencyPayment(GatepayErrorInfo),

    /// Check whether the callback address provided by the merchant is valid
    #[error("Check whether the callback address provided by the merchant is valid {0:?}")]
    #[serde(rename = "400624")]
    InvalidOrderStatusNotificationAddress(GatepayErrorInfo),

    /// Check whether the requested merchant ID is correct
    #[error("Check whether the requested merchant ID is correct {0:?}")]
    #[serde(rename = "500008")]
    CorrespondingMerchantNotFound(GatepayErrorInfo),

    /// Redefine and generate a new QR code
    #[error("Redefine and generate a new QR code {0:?}")]
    #[serde(rename = "500100")]
    PaymentQrCodeExpired(GatepayErrorInfo),

    /// Please verify the order status
    #[error("Please verify the order status {0:?}")]
    #[serde(rename = "500101")]
    DuplicatePaymentQrCode(GatepayErrorInfo),

    /// Exchange rate fluctuations affect the collection rate
    #[error("Exchange rate fluctuations affect the collection rate {0:?}")]
    #[serde(rename = "500103")]
    AddressPaymentExchangeCurrencyError(GatepayErrorInfo),

    /// Please check whether the address is correct
    #[error("Please check whether the address is correct {0:?}")]
    #[serde(rename = "500203")]
    UnableToQueryOrderDetailsForAddressPayment(GatepayErrorInfo),

    /// Please confirm that the recipient of the refund is a Gate user
    #[error("Please confirm that the recipient of the refund is a Gate user {0:?}")]
    #[serde(rename = "500204")]
    InvalidRecipientIdForRefundTransactionOrder(GatepayErrorInfo),

    /// Please ensure that the refund currency is one of the order currency or the user's payment currency
    #[error("Please ensure that the refund currency is one of the order currency or the user's payment currency {0:?}")]
    #[serde(rename = "500205")]
    RefundCurrencyDoesNotMatch(GatepayErrorInfo),

    /// Please check the refund amount of the order
    #[error("Please check the refund amount of the order {0:?}")]
    #[serde(rename = "500206")]
    RefundAmountExceedsLimit(GatepayErrorInfo),

    /// Please confirm whether the refund was successful or check whether the address is correct
    #[error("Please confirm whether the refund was successful or check whether the address is correct {0:?}")]
    #[serde(rename = "500207")]
    UnableToFindTheRefundOrderForAddressPayment(GatepayErrorInfo),

    /// Please confirm the type of refund order
    #[error("Please confirm the type of refund order {0:?}")]
    #[serde(rename = "500208")]
    CannotRefundOrdersWithoutAConvertedAddress(GatepayErrorInfo),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::error::GatepayResponse;
    use crate::api::error::GatepayResult;

    const FAILED_RESPONSE: &str = r#"{
        "status": "FAIL",
        "code": "400002",
        "label": "INVALID_SIGNATURE",
        "errorMessage": "Incorrect signature result",
        "data": {}
    }"#;

    const SUCCESSFUL_RESPONSE: &str = r#"{
        "status": "SUCCESS",
        "code": "000000",
        "errorMessage": "",
        "data": {
            "prepayId": "43013197477711872",
            "merchantId": 10002,
            "merchantTradeNo": "13683379532935164644",
            "currency": "USDT",
            "totalFee": "1.6",
            "merchant_name": "MINIAPP PAYMENT TEST",
            "goods_name": "NFT",
            "status": "PENDING",
            "qrcode": "http://openplatform.gate.io/qr/P_6uSR4icI56VUdM2lbYdVihLxR_SsrcNfbdzNzfgp0=",
            "create_time": 1672216745425,
            "expire_time": 1672220345420
        }
    }"#;

    #[test]
    fn test_failed_response() {
        let sample = GatepayErrorInfo {
            label: "INVALID_SIGNATURE".to_string(),
            error_message: "Incorrect signature result".to_string(),
        };

        let resp =
            serde_json::from_str::<GatepayResult<GatepayResponse<()>>>(FAILED_RESPONSE).unwrap();

        assert_eq!(
            resp,
            GatepayResult::Fail(GatepayApiError::InvalidSignature(sample))
        );
    }

    #[test]
    fn test_successful_response() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Test {
            prepay_id: String,
            merchant_id: u64,
            merchant_trade_no: String,
            currency: String,
        }

        let sample = Test {
            prepay_id: "43013197477711872".to_string(),
            merchant_id: 10002,
            merchant_trade_no: "13683379532935164644".to_string(),
            currency: "USDT".to_string(),
        };

        let resp = serde_json::from_str::<GatepayResult<Test>>(SUCCESSFUL_RESPONSE).unwrap();

        assert_eq!(
            resp,
            GatepayResult::Success(GatepayResponse { data: sample })
        );
    }
}
