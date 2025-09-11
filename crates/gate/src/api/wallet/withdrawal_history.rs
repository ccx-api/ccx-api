use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PrivateRequest;
use crate::api::Request;
use crate::api::withdrawal::WithdrawalWithdrawStatus;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct WalletWithdrawalHistoryRequest {
    /// Filter by currency. Return all currency records if not specified
    pub currency: Option<SmartString>,
    /// Time range beginning, default to 7 days before current time
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub from: Option<DateTime<Utc>>,
    /// Time range ending, default to current time
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub to: Option<DateTime<Utc>>,
    /// Maximum number of records to be returned in a single list
    pub limit: Option<u64>,
    /// List offset, starting from 0
    pub offset: Option<u64>,
}

impl Request for WalletWithdrawalHistoryRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = Vec<WalletWithdrawalHistoryResponse>;
}

impl PrivateRequest for WalletWithdrawalHistoryRequest {}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WalletWithdrawalHistoryResponse {
    /// Record ID
    pub id: SmartString,
    /// Hash record of the withdrawal
    pub txid: Option<SmartString<64>>,
    /// Client order id, up to 32 length and can only include 0-9, A-Z, a-z, underscore(_), hyphen(-) or dot(.)
    pub withdraw_order_id: Option<SmartString<32>>,
    /// Operation time
    #[serde_as(as = "Option<TimestampSeconds<i64, Flexible>>")]
    pub timestamp: Option<DateTime<Utc>>,
    /// Currency amount
    pub amount: Decimal,
    /// fee
    pub fee: Decimal,
    /// Currency name
    pub currency: SmartString,
    /// Withdrawal address. Required for withdrawals
    pub address: SmartString<66>,
    /// Additional remarks with regards to the withdrawal
    pub memo: Option<SmartString>,
    /// Record status.
    pub status: WithdrawalWithdrawStatus,
    /// Name of the chain used in withdrawals
    pub chain: SmartString,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::api::wallet::WalletApi;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;

    impl<S: GateSigner> WalletApi<S> {
        /// # Retrieve withdrawal records
        ///
        /// Retrieve withdrawal records
        ///
        /// Record time range cannot exceed 30 days
        /// ## Parameters
        ///
        /// * `currency` - Filter by currency. Return all currency records if not specified
        /// * `from` - Time range beginning, default to 7 days before current time
        /// * `to` - Time range ending, default to current time
        /// * `limit` - Maximum number of records to be returned in a single list
        /// * `offset` - List offset, starting from 0
        pub async fn withdrawal_history(
            &self,
            request: &WalletWithdrawalHistoryRequest,
        ) -> Result<<WalletWithdrawalHistoryRequest as Request>::Response, RequestError> {
            self.0.signed_request("/wallet/withdrawals", request).await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_example_from_docs() {
        let json = r#"[
        {
            "id": "210496",
            "timestamp": "1542000000",
            "withdraw_order_id": "order_123456",
            "currency": "USDT",
            "address": "1HkxtBAMrA3tP5ENnYY2CZortjZvFDH5Cs",
            "txid": "128988928203223323290",
            "amount": "222.61",
            "fee": "0.01",
            "memo": "",
            "status": "DONE",
            "chain": "TRX"
        }
    ]"#;
        let res: Vec<WalletWithdrawalHistoryResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            vec![WalletWithdrawalHistoryResponse {
                id: "210496".into(),
                timestamp: DateTime::from_timestamp(1542000000, 0),
                withdraw_order_id: Some("order_123456".into()),
                currency: "USDT".into(),
                address: "1HkxtBAMrA3tP5ENnYY2CZortjZvFDH5Cs".into(),
                txid: Some("128988928203223323290".into()),
                amount: dec!(222.61),
                fee: dec!(0.01),
                memo: Some("".into()),
                status: WithdrawalWithdrawStatus::Done,
                chain: "TRX".into(),
            }]
        );
    }
}
