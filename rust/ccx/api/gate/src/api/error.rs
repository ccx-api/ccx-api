use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GateErrorInfo {
    #[serde(default)]
    pub message: SmartString<104>,
}

/// [source](https://www.gate.io/docs/developers/apiv4/en/#label-list)
///
/// ## Request parameter or format related.
/// | label | Meaning |
/// | --- | --- |
/// | INVALID_PARAM_VALUE | Invalid parameter value |
/// | INVALID_PROTOCOL | Invalid parameter value |
/// | INVALID_ARGUMENT | Invalid argument |
/// | INVALID_REQUEST_BODY | Invalid request body |
/// | MISSING_REQUIRED_PARAM | Missing required parameter |
/// | BAD_REQUEST | Invalid request |
/// | INVALID_CONTENT_TYPE | Invalid Content-Type header |
/// | NOT_ACCEPTABLE | Invalid Accept- Header |
/// | METHOD_NOT_ALLOWED | Request method is not allowed |
/// | NOT_FOUND | Request URL not exists |
///
/// ## Authentication related
///
/// | label | Meaning |
/// | --- | --- |
/// | INVALID_CREDENTIALS | Invalid credentials provided |
/// | INVALID_KEY | Invalid API Key |
/// | IP_FORBIDDEN | Request IP not in whitelist |
/// | READ_ONLY | API key is read-only |
/// | INVALID_SIGNATURE | Invalid signature |
/// | MISSING_REQUIRED_HEADER | Missing required authentication header |
/// | REQUEST_EXPIRED | Request Timestamp is far from the server time |
/// | ACCOUNT_LOCKED | Account is locked |
/// | FORBIDDEN | Account has no permission to request operation |
///
/// ## Wallet related
///
/// | label | Meaning |
/// | --- | --- |
/// | SUB_ACCOUNT_NOT_FOUND | Sub account not found |
/// | SUB_ACCOUNT_LOCKED | Sub account is locked |
/// | MARGIN_BALANCE_EXCEPTION | Abnormal margin account |
/// | MARGIN_TRANSFER_FAILED | Failed to transfer with margin account |
/// | TOO_MUCH_FUTURES_AVAILABLE | Futures balance exceeds max allowed |
/// | FUTURES_BALANCE_NOT_ENOUGH | Futures balance not enough |
/// | ACCOUNT_EXCEPTION | Abnormal account |
/// | SUB_ACCOUNT_TRANSFER_FAILED | Failed to transfer with sub account |
/// | ADDRESS_NOT_USED | Address never being used in web console |
/// | TOO_FAST | Withdrawing request exceeds frequency limit |
/// | WITHDRAWAL_OVER_LIMIT | Withdrawal limit exceeded |
/// | API_WITHDRAW_DISABLED | API withdrawal operation is disabled temporarily |
/// | INVALID_WITHDRAW_ID | Invalid withdraw ID |
/// | INVALID_WITHDRAW_CANCEL_STATUS | Cancelling withdrawal not allowed with current status |
/// | DUPLICATE_REQUEST | Duplicate request |
/// | ORDER_EXISTS | Order already exists, do not resubmit |
/// | INVALID_CLIENT_ORDER_ID | The client_order_id is invalid |
///
/// ## Spot and margin trading related
///
/// | label | Meaning |
/// | --- | --- |
/// | INVALID_PRECISION | Invalid precision |
/// | INVALID_CURRENCY | Invalid currency |
/// | INVALID_CURRENCY_PAIR | Invalid currency pair |
/// | POC_FILL_IMMEDIATELY | Order would match and take immediately so it's cancelled |
/// | ORDER_NOT_FOUND | Order not found |
/// | ORDER_CLOSED | Order already closed |
/// | ORDER_CANCELLED | Order already cancelled |
/// | QUANTITY_NOT_ENOUGH | Amount is not enough |
/// | BALANCE_NOT_ENOUGH | Balance is not enough |
/// | MARGIN_NOT_SUPPORTED | Request currency pair doesn't provide margin trading |
/// | MARGIN_BALANCE_NOT_ENOUGH | Margin balance is not enough |
/// | AMOUNT_TOO_LITTLE | Amount does not reach minimum required |
/// | AMOUNT_TOO_MUCH | Amount exceeds maximum allowed |
/// | REPEATED_CREATION | Repeated creation |
/// | LOAN_NOT_FOUND | Margin loan is not found |
/// | LOAN_RECORD_NOT_FOUND | Margin loan record is not found |
/// | NO_MATCHED_LOAN | No loan can match request borrow requirement |
/// | NOT_MERGEABLE | Request loans cannot be merged |
/// | NO_CHANGE | No change is made |
/// | REPAY_TOO_MUCH | Repay more than required |
/// | TOO_MANY_CURRENCY_PAIRS | Too many currency pairs in batch orders creation |
/// | TOO_MANY_ORDERS | Too many orders in one currency pair in batch orders creation |
/// | MIXED_ACCOUNT_TYPE | More than one account type is used in batch orders creation |
/// | AUTO_BORROW_TOO_MUCH | Auto borrow exceeds maximum allowed |
/// | TRADE_RESTRICTED | Trading is restricted due to high debt ratio |
/// | FOK_NOT_FILL | FOK order cannot be filled completely |
/// | INITIAL_MARGIN_TOO_LOW | User's total initial margin rate is too low |
/// | NO_MERGEABLE_ORDERS | Orders can be merged not found |
/// | ORDER_BOOK_NOT_FOUND | Insufficient liquidity |
/// | FAILED_RETRIEVE_ASSETS | Failed to retrieve account assets |

// TODO Futures related
// TODO Collateral Loan related

///
/// ## Portfolio related
///
/// | label | Meaning |
/// | --- | --- |
/// | USER_LIAB | User has liab |
/// | USER_PENDING_ORDERS | User has pending orders |
/// | MODE_SET | already set portfolio_margin mode |
///
/// ## Server errors
///
/// | label | Meaning |
/// | --- | --- |
/// | INTERNAL | Internal server error |
/// | SERVER_ERROR | Internal server error |
/// | TOO_BUSY | Server is too busy at the moment |

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Error)]
#[serde(tag = "label")]
pub enum GateApiError {
    /*
     * Request parameter or format related.
     */
    /// Invalid parameter value
    #[error("Invalid parameter value {0:?}")]
    #[serde(rename = "INVALID_PARAM_VALUE")]
    InvalidParamValue(GateErrorInfo),

    /// Invalid parameter value
    #[error("Invalid parameter value {0:?}")]
    #[serde(rename = "INVALID_PROTOCOL")]
    InvalidProtocol(GateErrorInfo),

    /// Invalid argument
    #[error("Invalid argument {0:?}")]
    #[serde(rename = "INVALID_ARGUMENT")]
    InvalidArgument(GateErrorInfo),

    /// Invalid request body
    #[error("Invalid request body {0:?}")]
    #[serde(rename = "INVALID_REQUEST_BODY")]
    InvalidRequestBody(GateErrorInfo),

    /// Missing required parameter
    #[error("Missing required parameter {0:?}")]
    #[serde(rename = "MISSING_REQUIRED_PARAM")]
    MissingRequiredParam(GateErrorInfo),

    /// Invalid request
    #[error("Invalid request {0:?}")]
    #[serde(rename = "BAD_REQUEST")]
    BadRequest(GateErrorInfo),

    /// Invalid Content-Type header
    #[error("Invalid Content-Type header {0:?}")]
    #[serde(rename = "INVALID_CONTENT_TYPE")]
    InvalidContentType(GateErrorInfo),

    /// Invalid Accept Header
    #[error("Invalid Accept Header {0:?}")]
    #[serde(rename = "NOT_ACCEPTABLE")]
    NotAcceptable(GateErrorInfo),

    /// Request method is not allowed
    #[error("Request method is not allowed {0:?}")]
    #[serde(rename = "METHOD_NOT_ALLOWED")]
    MethodNotAllowed(GateErrorInfo),

    /// Request URL not exists
    #[error("Request URL not exists {0:?}")]
    #[serde(rename = "NOT_FOUND")]
    NotFound(GateErrorInfo),

    /*
     * Authentication related
     */
    /// Invalid credentials provided
    #[error("Invalid credentials provided {0:?}")]
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials(GateErrorInfo),

    /// Invalid API Key
    #[error("Invalid API Key {0:?}")]
    #[serde(rename = "INVALID_KEY")]
    InvalidKey(GateErrorInfo),

    /// Request IP not in whitelist
    #[error("Request IP not in whitelist {0:?}")]
    #[serde(rename = "IP_FORBIDDEN")]
    IpForbidden(GateErrorInfo),

    /// API key is read-only
    #[error("API key is read-only {0:?}")]
    #[serde(rename = "READ_ONLY")]
    ReadOnly(GateErrorInfo),

    /// Invalid signature
    #[error("Invalid signature {0:?}")]
    #[serde(rename = "INVALID_SIGNATURE")]
    InvalidSignature(GateErrorInfo),

    /// Missing required authentication header
    #[error("Missing required authentication header {0:?}")]
    #[serde(rename = "MISSING_REQUIRED_HEADER")]
    MissingRequiredHeader(GateErrorInfo),

    /// Request Timestamp is far from the server time
    #[error("Request Timestamp is far from the server time {0:?}")]
    #[serde(rename = "REQUEST_EXPIRED")]
    RequestExpired(GateErrorInfo),

    /// Account is locked
    #[error("Account is locked {0:?}")]
    #[serde(rename = "ACCOUNT_LOCKED")]
    AccountLocked(GateErrorInfo),

    /// Account has no permission to request operation
    #[error("Account has no permission to request operation {0:?}")]
    #[serde(rename = "FORBIDDEN")]
    Forbidden(GateErrorInfo),

    /*
     * Wallet related
     */
    /// Sub account not found
    #[error("Sub account not found {0:?}")]
    #[serde(rename = "SUB_ACCOUNT_NOT_FOUND")]
    SubAccountNotFound(GateErrorInfo),

    /// Sub account is locked
    #[error("Sub account is locked {0:?}")]
    #[serde(rename = "SUB_ACCOUNT_LOCKED")]
    SubAccountLocked(GateErrorInfo),

    /// Abnormal margin account
    #[error("Abnormal margin account {0:?}")]
    #[serde(rename = "MARGIN_BALANCE_EXCEPTION")]
    MarginBalanceException(GateErrorInfo),

    /// Failed to transfer with margin account
    #[error("Failed to transfer with margin account {0:?}")]
    #[serde(rename = "MARGIN_TRANSFER_FAILED")]
    MarginTransferFailed(GateErrorInfo),

    /// Futures balance exceeds max allowed
    #[error("Futures balance exceeds max allowed {0:?}")]
    #[serde(rename = "TOO_MUCH_FUTURES_AVAILABLE")]
    TooMuchFuturesAvailable(GateErrorInfo),

    /// Futures balance not enough
    #[error("Futures balance not enough {0:?}")]
    #[serde(rename = "FUTURES_BALANCE_NOT_ENOUGH")]
    FuturesBalanceNotEnough(GateErrorInfo),

    /// Abnormal account
    #[error("Abnormal account {0:?}")]
    #[serde(rename = "ACCOUNT_EXCEPTION")]
    AccountException(GateErrorInfo),

    /// Failed to transfer with sub account
    #[error("Failed to transfer with sub account {0:?}")]
    #[serde(rename = "SUB_ACCOUNT_TRANSFER_FAILED")]
    SubAccountTransferFailed(GateErrorInfo),

    /// Address never being used in web console
    #[error("Address never being used in web console {0:?}")]
    #[serde(rename = "ADDRESS_NOT_USED")]
    AddressNotUsed(GateErrorInfo),

    /// Withdrawing request exceeds frequency limit
    #[error("Withdrawing request exceeds frequency limit {0:?}")]
    #[serde(rename = "TOO_FAST")]
    TooFast(GateErrorInfo),

    /// Withdrawal limit exceeded
    #[error("Withdrawal limit exceeded {0:?}")]
    #[serde(rename = "WITHDRAWAL_OVER_LIMIT")]
    WithdrawalOverLimit(GateErrorInfo),

    /// API withdrawal operation is disabled temporarily
    #[error("API withdrawal operation is disabled temporarily {0:?}")]
    #[serde(rename = "API_WITHDRAW_DISABLED")]
    ApiWithdrawDisabled(GateErrorInfo),

    /// Invalid withdraw ID
    #[error("Invalid withdraw ID {0:?}")]
    #[serde(rename = "INVALID_WITHDRAW_ID")]
    InvalidWithdrawId(GateErrorInfo),

    /// Cancelling withdrawal not allowed with current status
    #[error("Cancelling withdrawal not allowed with current status {0:?}")]
    #[serde(rename = "INVALID_WITHDRAW_CANCEL_STATUS")]
    InvalidWithdrawCancelStatus(GateErrorInfo),

    /// Duplicate request
    #[error("Duplicate request {0:?}")]
    #[serde(rename = "DUPLICATE_REQUEST")]
    DuplicateRequest(GateErrorInfo),

    /// Order already exists, do not resubmit
    #[error("Order already exists, do not resubmit {0:?}")]
    #[serde(rename = "ORDER_EXISTS")]
    OrderExists(GateErrorInfo),

    /// The client_order_id is invalid
    #[error("The client_order_id is invalid {0:?}")]
    #[serde(rename = "INVALID_CLIENT_ORDER_ID")]
    InvalidClientOrderId(GateErrorInfo),

    /*
     * Spot and margin trading related
     */
    /// Invalid precision
    #[error("Invalid precision {0:?}")]
    #[serde(rename = "INVALID_PRECISION")]
    InvalidPrecision(GateErrorInfo),

    /// Invalid currency
    #[error("Invalid currency {0:?}")]
    #[serde(rename = "INVALID_CURRENCY")]
    InvalidCurrency(GateErrorInfo),

    /// Invalid currency pair
    #[error("Invalid currency pair {0:?}")]
    #[serde(rename = "INVALID_CURRENCY_PAIR")]
    InvalidCurrencyPair(GateErrorInfo),

    /// Order would match and take immediately so it's cancelled
    #[error("Order would match and take immediately so it's cancelled {0:?}")]
    #[serde(rename = "POC_FILL_IMMEDIATELY")]
    PocFillImmediately(GateErrorInfo),

    /// Order not found
    #[error("Order not found {0:?}")]
    #[serde(rename = "ORDER_NOT_FOUND")]
    OrderNotFound(GateErrorInfo),

    /// Order already closed
    #[error("Order already closed {0:?}")]
    #[serde(rename = "ORDER_CLOSED")]
    OrderClosed(GateErrorInfo),

    /// Order already cancelled
    #[error("Order already cancelled {0:?}")]
    #[serde(rename = "ORDER_CANCELLED")]
    OrderCancelled(GateErrorInfo),

    /// Amount is not enough
    #[error("Amount is not enough {0:?}")]
    #[serde(rename = "QUANTITY_NOT_ENOUGH")]
    QuantityNotEnough(GateErrorInfo),

    /// Balance is not enough
    #[error("Balance is not enough {0:?}")]
    #[serde(rename = "BALANCE_NOT_ENOUGH")]
    BalanceNotEnough(GateErrorInfo),

    /// Request currency pair doesn't provide margin trading
    #[error("Request currency pair doesn't provide margin trading {0:?}")]
    #[serde(rename = "MARGIN_NOT_SUPPORTED")]
    MarginNotSupported(GateErrorInfo),

    /// Margin balance is not enough
    #[error("Margin balance is not enough {0:?}")]
    #[serde(rename = "MARGIN_BALANCE_NOT_ENOUGH")]
    MarginBalanceNotEnough(GateErrorInfo),

    /// Amount does not reach minimum required
    #[error("Amount does not reach minimum required {0:?}")]
    #[serde(rename = "AMOUNT_TOO_LITTLE")]
    AmountTooLittle(GateErrorInfo),

    /// Amount exceeds maximum allowed
    #[error("Amount exceeds maximum allowed {0:?}")]
    #[serde(rename = "AMOUNT_TOO_MUCH")]
    AmountTooMuch(GateErrorInfo),

    /// Repeated creation
    #[error("Repeated creation {0:?}")]
    #[serde(rename = "REPEATED_CREATION")]
    RepeatedCreation(GateErrorInfo),

    /// Margin loan is not found
    #[error("Margin loan is not found {0:?}")]
    #[serde(rename = "LOAN_NOT_FOUND")]
    LoanNotFound(GateErrorInfo),

    /// Margin loan record is not found
    #[error("Margin loan record is not found {0:?}")]
    #[serde(rename = "LOAN_RECORD_NOT_FOUND")]
    LoanRecordNotFound(GateErrorInfo),

    /// No loan can match request borrow requirement
    #[error("No loan can match request borrow requirement {0:?}")]
    #[serde(rename = "NO_MATCHED_LOAN")]
    NoMatchedLoan(GateErrorInfo),

    /// Request loans cannot be merged
    #[error("Request loans cannot be merged {0:?}")]
    #[serde(rename = "NOT_MERGEABLE")]
    NotMergeable(GateErrorInfo),

    /// No change is made
    #[error("No change is made {0:?}")]
    #[serde(rename = "NO_CHANGE")]
    NoChange(GateErrorInfo),

    /// Repay more than required
    #[error("Repay more than required {0:?}")]
    #[serde(rename = "REPAY_TOO_MUCH")]
    RepayTooMuch(GateErrorInfo),

    /// Too many currency pairs in batch orders creation
    #[error("Too many currency pairs in batch orders creation {0:?}")]
    #[serde(rename = "TOO_MANY_CURRENCY_PAIRS")]
    TooManyCurrencyPairs(GateErrorInfo),

    /// Too many orders in one currency pair in batch orders creation
    #[error("Too many orders in one currency pair in batch orders creation {0:?}")]
    #[serde(rename = "TOO_MANY_ORDERS")]
    TooManyOrders(GateErrorInfo),

    /// More than one account type is used in batch orders creation
    #[error("More than one account type is used in batch orders creation {0:?}")]
    #[serde(rename = "MIXED_ACCOUNT_TYPE")]
    MixedAccountType(GateErrorInfo),

    /// Auto borrow exceeds maximum allowed
    #[error("Auto borrow exceeds maximum allowed {0:?}")]
    #[serde(rename = "AUTO_BORROW_TOO_MUCH")]
    AutoBorrowTooMuch(GateErrorInfo),

    /// Trading is restricted due to high debt ratio
    #[error("Trading is restricted due to high debt ratio {0:?}")]
    #[serde(rename = "TRADE_RESTRICTED")]
    TradeRestricted(GateErrorInfo),

    /// FOK order cannot be filled completely
    #[error("FOK order cannot be filled completely {0:?}")]
    #[serde(rename = "FOK_NOT_FILL")]
    FokNotFill(GateErrorInfo),

    /// User's total initial margin rate is too low
    #[error("User's total initial margin rate is too low {0:?}")]
    #[serde(rename = "INITIAL_MARGIN_TOO_LOW")]
    InitialMarginTooLow(GateErrorInfo),

    /// Orders can be merged not found
    #[error("Orders can be merged not found {0:?}")]
    #[serde(rename = "NO_MERGEABLE_ORDERS")]
    NoMergeableOrders(GateErrorInfo),

    /// Insufficient liquidity
    #[error("Insufficient liquidity {0:?}")]
    #[serde(rename = "ORDER_BOOK_NOT_FOUND")]
    OrderBookNotFound(GateErrorInfo),

    /// Failed to retrieve account assets
    #[error("Failed to retrieve account assets {0:?}")]
    #[serde(rename = "FAILED_RETRIEVE_ASSETS")]
    FailedRetrieveAssets(GateErrorInfo),

    /*
     * Portfolio related
     */
    /// User has liab
    #[error("User has liab {0:?}")]
    #[serde(rename = "USER_LIAB")]
    UserLiab(GateErrorInfo),

    /// User has pending orders
    #[error("User has pending orders {0:?}")]
    #[serde(rename = "USER_PENDING_ORDERS")]
    UserPendingOrders(GateErrorInfo),

    /// already set portfolio_margin mode
    #[error("already set portfolio_margin mode {0:?}")]
    #[serde(rename = "MODE_SET")]
    ModeSet(GateErrorInfo),

    /*
     * Server errors
     */
    /// Internal server error
    #[error("Internal server error {0:?}")]
    #[serde(rename = "SERVER_ERROR")]
    ServerError(GateErrorInfo),

    /// Internal server error
    #[error("Internal server error {0:?}")]
    #[serde(rename = "INTERNAL")]
    Internal(GateErrorInfo),

    /// Server is too busy at the moment
    #[error("Server is too busy at the moment {0:?}")]
    #[serde(rename = "TOO_BUSY")]
    TooBusy(GateErrorInfo),

    /*
     * Client errors
     */
    /// Client error, e.g. amount to transfer is bigger than balance.
    #[error("Internal server error {0:?}")]
    #[serde(rename = "CLIENT_ERROR")]
    ClientError(GateErrorInfo),
}

#[cfg(test)]
mod tests {
    use smart_string::DisplayExt;

    use super::*;

    const FAILED_RESPONSE: &str = r#"{
        "label": "INVALID_SIGNATURE",
        "message": "Invalid signature"
    }"#;

    #[test]
    fn test_failed_response() {
        let sample = GateErrorInfo {
            message: "Invalid signature".to_fmt(),
        };

        let resp = serde_json::from_str::<GateApiError>(FAILED_RESPONSE).unwrap();

        assert_eq!(resp, GateApiError::InvalidSignature(sample));
    }
}
