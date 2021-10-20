use super::prelude::*;

pub const SAPI_V1_FIAT_CLEARJUNCTION_WITHDRAW: &str = "/sapi/v1/fiat/clearjunction/withdraw";
pub const SAPI_V1_FIAT_CLEARJUNCTION_QUERY_TRANSACTION: &str =
    "/sapi/v1/fiat/clearjunction/queryTransaction";
pub const SAPI_V1_FIAT_CLEARJUNCTION_LIST_TRANSACTION: &str =
    "/sapi/v1/fiat/clearjunction/listTransaction";
pub const SAPI_V1_FIAT_CLEARJUNCTION_GET_BALANCE: &str = "/sapi/v1/fiat/clearjunction/getBalance";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearjunctionWithdraw {
    /// binance transaction id
    pub transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearjunctionTransaction {
    /// binance transaction id
    pub transaction_id: String,
    pub status: ClearjunctionWithdrawStatus,
    pub fail_reason: Option<String>,
    pub currency: Atom,
    pub amount: Decimal,
    pub fee: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClearjunctionWithdrawStatus {
    Pending,
    Succeed,
    Failed,
}

impl ClearjunctionWithdrawStatus {
    pub fn is_finished(&self) -> bool {
        use ClearjunctionWithdrawStatus as WS;
        matches!(
            self,
            WS::Succeed | WS::Failed
        )
    }

    pub fn is_succeed(&self) -> bool {
        use ClearjunctionWithdrawStatus as WS;
        matches!(
            self,
            WS::Succeed
        )
    }

    pub fn is_failed(&self) -> bool {
        use ClearjunctionWithdrawStatus as WS;
        matches!(
            self,
            WS::Failed
        )
    }

    pub fn is_pending(&self) -> bool {
        use ClearjunctionWithdrawStatus as WS;
        matches!(self, WS::Pending)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearjunctionTransactionList {
    pub count: u64,
    pub transaction_list: Vec<ClearjunctionTransaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearjunctionBalance {
    pub amount: Decimal,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl SpotApi {
        /// Submit withdraw `[SAPI]`
        ///
        /// Submit a new withdrawal.
        ///
        /// Weight: 1
        ///
        /// * initTime - Must be a 13 digit millisecond format.
        ///
        /// * Note.1 'initTime' field is different from 'timestamp' field, can retry sending request
        ///   with same initTime and different timestamp.
        /// * Note.2 Any transaction can only be retry within 15 mins, which means if 'initTime' is
        ///   already 15 mins ago, binance side will reject the request directly.
        /// * Note.3 Transaction frequency limit is 1 transaction per second. Any request with initTime
        ///   less then 1min from last one will be rejected.
        pub async fn clearjunction_withdraw(
            &self,
            currency: impl Serialize,
            amount: Decimal,
            init_time: u64,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<ClearjunctionWithdraw> {
            self.client
                .post(SAPI_V1_FIAT_CLEARJUNCTION_WITHDRAW)?
                .signed(time_window)?
                .query_arg("currency", &currency)?
                .query_arg("amount", &amount)?
                .query_arg("initTime", &init_time)?
                .send()
                .await
        }

        /// Query transaction `[USER_DATA]`
        ///
        /// Query a transaction's latest status under current user.
        ///
        /// Weight: 1
        ///
        /// * transactionId - the same id from withdraw response.
        pub async fn clearjunction_query_transaction(
            &self,
            transaction_id: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<ClearjunctionTransaction> {
            self.client
                .get(SAPI_V1_FIAT_CLEARJUNCTION_QUERY_TRANSACTION)?
                .signed(time_window)?
                .query_arg("transactionId", &transaction_id)?
                .send()
                .await
        }

        /// List transactions `[USER_DATA]`
        ///
        /// List latest status of all trasnactions under currrent user created during a time period.
        ///
        /// Weight: 1
        ///
        /// * transactionId - the same id from withdraw response.
        pub async fn clearjunction_list_transaction(
            &self,
            begin_time: u64,
            end_time: u64,
            currency: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<ClearjunctionTransaction> {
            self.client
                .get(SAPI_V1_FIAT_CLEARJUNCTION_LIST_TRANSACTION)?
                .signed(time_window)?
                .query_arg("beginTime", &begin_time)?
                .query_arg("endTime", &end_time)?
                .try_query_arg("currency", &currency)?
                .send()
                .await
        }

        /// Check balance `[USER_DATA]`
        ///
        /// Get user walllet balance.
        ///
        /// Weight: 1
        pub async fn clearjunction_get_balance(
            &self,
            currency: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<ClearjunctionBalance> {
            self.client
                .get(SAPI_V1_FIAT_CLEARJUNCTION_GET_BALANCE)?
                .signed(time_window)?
                .query_arg("currency", &currency)?
                .send()
                .await
        }
    }
}
