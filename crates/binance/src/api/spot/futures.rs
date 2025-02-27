use super::prelude::*;
use super::RL_WEIGHT_PER_MINUTE;
use crate::client::Task;

pub const SAPI_V1_FUTURES_TRANSFER: &str = "/sapi/v1/futures/transfer";
pub const SAPI_V2_FUTURES_LOAN_CONFIGS: &str = "/sapi/v1/futures/loan/configs";

/// [https://binance-docs.github.io/apidocs/spot/en/#new-future-account-transfer-user_data]
#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[repr(u32)]
pub enum FuturesTransferType {
    /// Transfer from spot account to USDT-Ⓜ futures account.
    Spot2Usdtm = 1,
    /// Transfer from USDT-Ⓜ futures account to spot account.
    Usdtm2Spot = 2,
    /// Transfer from spot account to COIN-Ⓜ futures account.
    Spot2Coinm = 3,
    /// Transfer from COIN-Ⓜ futures account to spot account.
    Coinm2Spot = 4,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewFuturesAccountTransfer {
    /// Transaction id.
    pub tran_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountTransferHistoryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    rows: Vec<FuturesAccountTransfer>,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountTransfer {
    pub asset: Atom,
    /// Transaction id.
    pub tran_id: u64,
    pub amount: Decimal,
    pub r#type: FuturesTransferType,
    pub timestamp: u64,
    pub status: FuturesTransferStatus,
}

/// [https://binance-docs.github.io/apidocs/spot/en/#new-future-account-transfer-user_data]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FuturesTransferStatus {
    /// Pending to execution.
    #[serde(rename = "PENDING")]
    Pending,
    /// Successfully transfered.
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    /// Execution failed, nothing happened to your account.
    #[serde(rename = "FAILED")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesCrossCollateranlInformationV2 {
    pub loan_coin: Atom,
    pub collateral_coin: Atom,
    pub rate: Decimal,
    pub margin_call_collateral_rate: Decimal,
    pub liquidation_collateral_rate: Decimal,
    pub current_collateral_rate: Decimal,
    /// New for interest collection.
    #[serde(default, skip_serializing_if = "Decimal::is_zero")]
    pub interest_rate: Decimal,
    /// Days, new for interest collection.
    pub interest_grace_period: String,
    // pub interest_grace_period: Decimal,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> SpotApi<S>
    where
        S: crate::client::BinanceSigner,
        S: Unpin + 'static,
    {
        /// New Future Account Transfer (USER_DATA).
        ///
        /// Execute transfer between spot account and futures account.
        ///
        /// Weight(IP): 1
        ///
        /// * asset - The asset being transferred, e.g., USDT.
        /// * amount - The amount to be transferred.
        pub fn futures_create_transfer(
            &self,
            asset: impl Serialize,
            amount: Decimal,
            r#type: FuturesTransferType,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<NewFuturesAccountTransfer>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_FUTURES_TRANSFER)?
                        .signed(time_window)?
                        .query_arg("asset", &asset)?
                        .query_arg("amount", &amount)?
                        .query_arg("type", &r#type)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Get Future Account Transaction History List (USER_DATA).
        ///
        /// Weight(IP): 10
        ///
        /// * current_page - Start from 1. Default: 1.
        /// * page_size - Default: 10 Max: 100.
        pub fn futures_transfer_history(
            &self,
            asset: impl Serialize,
            start_time: u64,
            end_time: Option<u64>,
            current_page: Option<u64>,
            page_size: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<FuturesAccountTransferHistoryList>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_FUTURES_TRANSFER)?
                        .signed(time_window)?
                        .query_arg("asset", &asset)?
                        .query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?
                        .try_query_arg("current", &current_page)?
                        .try_query_arg("size", &page_size)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        // TODO Borrow For Cross-Collateral (TRADE)
        // TODO Cross-Collateral Borrow History (USER_DATA)
        // TODO Repay For Cross-Collateral (TRADE)
        // TODO Cross-Collateral Repayment History (USER_DATA)
        // TODO Cross-Collateral Wallet (USER_DATA)
        // TODO Cross-Collateral Wallet V2 (USER_DATA)
        // TODO Cross-Collateral Information (USER_DATA)

        /// Cross-Collateral Information V2 (USER_DATA)
        ///
        /// Weight(IP): 10
        ///
        /// * All loan and collateral data will be returned if loanCoin or collateralCoin is not sent.
        pub fn futures_cross_collateral_info_v2(
            &self,
            loan_coin: Option<impl Serialize>,
            collateral_coin: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<FuturesCrossCollateranlInformationV2>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V2_FUTURES_LOAN_CONFIGS)?
                        .signed(time_window)?
                        .try_query_arg("loanCoin", &loan_coin)?
                        .try_query_arg("collateralCoin", &collateral_coin)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        // TODO Calculate Rate After Adjust Cross-Collateral LTV (USER_DATA)
        // TODO Calculate Rate After Adjust Cross-Collateral LTV V2 (USER_DATA)
        // TODO Get Max Amount for Adjust Cross-Collateral LTV (USER_DATA)
        // TODO Get Max Amount for Adjust Cross-Collateral LTV V2 (USER_DATA)
        // TODO Adjust Cross-Collateral LTV (TRADE)
        // TODO Adjust Cross-Collateral LTV V2 (TRADE)
        // TODO Adjust Cross-Collateral LTV History (USER_DATA)
        // TODO Cross-Collateral Liquidation History (USER_DATA)
        // TODO Check Collateral Repay Limit (USER_DATA)
        // TODO Get Collateral Repay Quote (USER_DATA)
        // TODO Repay with Collateral (USER_DATA)
        // TODO Collateral Repayment Result (USER_DATA)
        // TODO Cross-Collateral Interest History (USER_DATA)
    }
}
