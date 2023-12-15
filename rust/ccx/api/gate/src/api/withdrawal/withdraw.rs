use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;
use crate::util::dt_gate::DtGate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalWithdrawRequest {
    /// Client order id, up to 32 length and can only include 0-9, A-Z, a-z, underscore(_), hyphen(-) or dot(.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<SmartString<32>>,
    /// Currency amount
    pub amount: Decimal,
    /// Currency name
    pub currency: SmartString,
    /// Withdrawal address. Required for withdrawals
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<SmartString<66>>,
    /// Additional remarks with regards to the withdrawal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<SmartString>,
    /// Name of the chain used in withdrawals
    pub chain: SmartString,
}

impl Request for WithdrawalWithdrawRequest {
    const METHOD: ApiMethod = ApiMethod::Post;
    const VERSION: ApiVersion = ApiVersion::V4;
    const PATH: &'static str = "withdrawals";
    const IS_PUBLIC: bool = false;
    type Response = WithdrawalWithdrawResponse;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalWithdrawResponse {
    /// Record ID
    pub id: SmartString,
    /// Hash record of the withdrawal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub txid: Option<SmartString<64>>,
    /// Client order id, up to 32 length and can only include 0-9, A-Z, a-z, underscore(_), hyphen(-) or dot(.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<SmartString<32>>,
    /// Operation time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DtGate>,
    /// Currency amount
    pub amount: Decimal,
    /// Currency name
    pub currency: SmartString,
    /// Withdrawal address. Required for withdrawals
    pub address: SmartString<66>,
    /// Additional remarks with regards to the withdrawal
    #[serde(default)]
    pub memo: Option<SmartString>,
    /// Record status.
    pub status: WithdrawalWithdrawStatus,
    /// Name of the chain used in withdrawals
    pub chain: SmartString,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WithdrawalWithdrawStatus {
    /// done
    Done,
    /// cancelled
    Cancel,
    /// requesting
    Request,
    /// pending manual approval
    Manual,
    /// GateCode operation
    Bcode,
    /// pending confirm after sending
    Extpend,
    /// pending confirm when fail
    Fail,
    /// invalid order
    Invalid,
    /// verifying
    Verify,
    /// processing
    Proces,
    /// pending
    Pend,
    /// required manual approval
    Dmove,
    /// the order is automatically split due to large amount
    Splitpend,
}

impl WithdrawalWithdrawStatus {
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            WithdrawalWithdrawStatus::Done
                | WithdrawalWithdrawStatus::Cancel
                | WithdrawalWithdrawStatus::Fail
        )
    }

    pub fn is_pending(&self) -> bool {
        matches!(
            self,
            WithdrawalWithdrawStatus::Request
                | WithdrawalWithdrawStatus::Manual
                | WithdrawalWithdrawStatus::Bcode
                | WithdrawalWithdrawStatus::Extpend
                | WithdrawalWithdrawStatus::Verify
                | WithdrawalWithdrawStatus::Proces
                | WithdrawalWithdrawStatus::Pend
                | WithdrawalWithdrawStatus::Splitpend
        )
    }

    pub fn needs_confirmation(&self) -> bool {
        matches!(self, WithdrawalWithdrawStatus::Manual)
    }
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;
    use crate::GateApi;

    impl<S: GateSigner> GateApi<S> {
        /// # Withdraw
        ///
        /// Withdraw
        ///
        /// ‼️ Be aware that Client order id does not guarantee the uniqueness of the order id on the Gate API side.
        ///
        /// ## Parameters
        ///
        /// * `withdraw_order_id` - Client order id, up to 32 length and can only include 0-9, A-Z, a-z, underscore(_),
        ///   hyphen(-) or dot(.)
        /// * `amount` - Currency amount
        /// * `currency` - Currency name
        /// * `address` - Withdrawal address. Required for withdrawals
        /// * `memo` - Additional remarks with regards to the withdrawal
        /// * `chain` - Name of the chain used in withdrawals
        pub async fn withdrawal_withdraw(
            &self,
            withdraw_order_id: Option<SmartString<32>>,
            amount: Decimal,
            currency: SmartString,
            address: Option<SmartString<66>>,
            memo: Option<SmartString>,
            chain: SmartString,
        ) -> Result<<WithdrawalWithdrawRequest as Request>::Response, RequestError> {
            self.request(&WithdrawalWithdrawRequest {
                withdraw_order_id,
                amount,
                currency,
                address,
                memo,
                chain,
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
    fn test_example_from_docs() {
        let json = r#"{
            "id": "210496",
            "timestamp": "1542000000",
            "withdraw_order_id": "order_123456",
            "currency": "USDT",
            "address": "1HkxtBAMrA3tP5ENnYY2CZortjZvFDH5Cs",
            "txid": "128988928203223323290",
            "amount": "222.61",
            "memo": "",
            "status": "DONE",
            "chain": "TRX"
        }"#;
        let res: WithdrawalWithdrawResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            WithdrawalWithdrawResponse {
                id: "210496".into(),
                timestamp: Some(DtGate::from_timestamp(1542000000)),
                withdraw_order_id: Some("order_123456".into()),
                currency: "USDT".into(),
                address: "1HkxtBAMrA3tP5ENnYY2CZortjZvFDH5Cs".into(),
                txid: Some("128988928203223323290".into()),
                amount: dec!(222.61),
                memo: Some("".into()),
                status: WithdrawalWithdrawStatus::Done,
                chain: "TRX".into(),
            }
        );
    }

    #[test]
    fn test_example_real_response() {
        let json = r#"{"id":"w50000000","currency":"USDT","amount":"2.63","address":"Txxx","memo":null,"status":"REQUEST","chain":"TRX","withdraw_order_id":"47eaed6f32f24cb7a765fef1966e775b"}"#;
        let res: WithdrawalWithdrawResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            WithdrawalWithdrawResponse {
                id: "w50000000".into(),
                timestamp: None,
                withdraw_order_id: Some("47eaed6f32f24cb7a765fef1966e775b".into()),
                currency: "USDT".into(),
                address: "Txxx".into(),
                txid: None,
                amount: dec!(2.63),
                memo: None,
                status: WithdrawalWithdrawStatus::Request,
                chain: "TRX".into(),
            }
        );
    }
}
